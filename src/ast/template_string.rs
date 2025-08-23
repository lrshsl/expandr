use crate::{errs::ParsingError, lexer::RawToken};

use super::*;

#[derive(Debug)]
pub struct TemplateString<'s> {
    pieces: Vec<TemplatePiece<'s>>,
}

impl<'s> Expandable<'s> for TemplateString<'s> {
    fn expand(&self, ctx: &'s ProgramContext) -> String {
        self.pieces
            .iter()
            .map(|piece| match piece {
                TemplatePiece::Expr(expr) => expr.expand(ctx),
                TemplatePiece::StrVal(s) => s.to_string(),
            })
            .collect()
    }
}

impl<'s> TemplateString<'s> {
    pub fn parse(
        parser: &mut Parser<'s>,
        number_delimiters: usize,
    ) -> Result<Self, ParsingError<'s>> {
        let mut pieces = Vec::new();

        parser.advance();
        parser.lex_raw_mode();
        loop {
            match parser.unpack_raw_token() {
                RawToken::RawPart(s) => {
                    pieces.push(TemplatePiece::StrVal(s));
                    parser.advance_raw();
                }
                RawToken::ExprStart => pieces.push(TemplatePiece::Expr(Expr::parse(parser)?)),
                RawToken::TemplateStringDelimiter(n) if n == number_delimiters => {
                    (0..n).for_each(|_| parser.advance_raw());
                    break;
                }
                RawToken::TemplateStringDelimiter(_) => {
                    pieces.push(TemplatePiece::StrVal(parser.raw_lexer.slice()));
                }
            }
        }
        parser.lex_expr_mode();

        parser.advance();
        Ok(Self { pieces })
    }
}

#[derive(Debug)]
pub enum TemplatePiece<'s> {
    StrVal(&'s str),
    Expr(Expr<'s>),
}

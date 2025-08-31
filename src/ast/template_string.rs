use crate::{errs::ParsingError, lexer::RawToken, log_lexer, parser::ParseMode};

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

        parser.switch_mode(ParseMode::Raw);
        parser.advance();
        loop {
            match parser
                .current_raw()
                .expect("TemplateString::parse on no token")
            {
                RawToken::RawPart(s) => {
                    pieces.push(TemplatePiece::StrVal(s));
                    parser.advance();
                }
                RawToken::EscapedOpeningBracket => {
                    pieces.push(TemplatePiece::StrVal("["));
                    parser.advance();
                }
                RawToken::ExprStart => {
                    log_lexer!("ExprStart 1: {:?}", parser.current_raw());
                    parser.switch_mode(ParseMode::Expr);
                    parser.advance();
                    log_lexer!("ExprStart 2: {:?}", parser.current_expr());
                    pieces.push(TemplatePiece::Expr(Expr::parse(parser)?));
                    log_lexer!("ExprStart 3: {:?}", parser.current_expr());
                    parser.switch_mode(ParseMode::Raw);
                }
                RawToken::TemplateStringDelimiter(n) if n == number_delimiters => {
                    break;
                }
                RawToken::TemplateStringDelimiter(_) => {
                    pieces.push(TemplatePiece::StrVal(parser.raw_lexer.slice()));
                }
            }
        }
        parser.switch_mode(ParseMode::Expr);
        parser.advance();

        Ok(Self { pieces })
    }
}

#[derive(Debug)]
pub enum TemplatePiece<'s> {
    StrVal(&'s str),
    Expr(Expr<'s>),
}

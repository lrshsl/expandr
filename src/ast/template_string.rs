use crate::{errs::ParsingError, lexer::RawToken, log_lexer, parser::ParseMode};

use super::*;

#[derive(Clone, Debug)]
pub struct TemplateString<'s> {
    pieces: Vec<TemplatePiece<'s>>,
}

impl<'s> Expandable<'s> for TemplateString<'s> {
    fn expand(&self, ctx: &'s ProgramContext) -> String {
        self.pieces.iter().map(|piece| piece.expand(ctx)).collect()
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
                    eprint!("'{s}' ");
                    pieces.push(TemplatePiece::StrVal(s));
                    parser.advance();
                }
                RawToken::EscapedOpeningBracket => {
                    eprint!("[");
                    pieces.push(TemplatePiece::StrVal("["));
                    parser.advance();
                }
                RawToken::ExprStart => {
                    eprint!(">> ");
                    parser.switch_mode(ParseMode::Expr);
                    parser.advance();
                    pieces.push(TemplatePiece::Expr(Expr::parse(parser, ParseMode::Raw)?));
                    parser.switch_mode(ParseMode::Raw);
                }
                RawToken::TemplateStringDelimiter(n) if n == number_delimiters => {
                    eprint!("TS_End >> ");
                    break;
                }
                RawToken::TemplateStringDelimiter(_) => {
                    eprint!("'{}' ", parser.raw_lexer.slice());
                    pieces.push(TemplatePiece::StrVal(parser.raw_lexer.slice()));
                }
            }
        }
        parser.switch_mode(ParseMode::Expr);
        parser.advance();

        Ok(Self { pieces })
    }
}

#[derive(Clone, Debug)]
pub enum TemplatePiece<'s> {
    StrVal(&'s str),
    Expr(Expr<'s>),
}

impl<'s> Expandable<'s> for TemplatePiece<'s> {
    fn expand(&self, ctx: &'s ProgramContext) -> String {
        match self {
            TemplatePiece::StrVal(s) => s.to_string(),
            TemplatePiece::Expr(e) => e.expand(ctx),
        }
    }
}

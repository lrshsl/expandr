use crate::{
    errors::parse_error::ParseError, expand::Expanded, lexer::RawToken, parser::ParseMode,
};

use super::*;

#[derive(Clone, Debug)]
pub struct TemplateString<'s> {
    pieces: Vec<TemplatePiece<'s>>,
}

impl<'s> Expandable<'s> for TemplateString<'s> {
    fn expand(self, ctx: &'s ProgramContext) -> Expanded {
        let mut result = String::new();
        for piece in self.pieces.into_iter() {
            match piece {
                TemplatePiece::Char(ch) => result.push(ch),
                TemplatePiece::StrVal(s) => result.push_str(s),
                TemplatePiece::Expr(expr) => result.push_str(&expr.expand(ctx).into_string()),
            }
        }
        Expanded::Str(result)
    }
}

impl<'s> TemplateString<'s> {
    pub fn parse(
        parser: &mut Parser<'s>,
        number_delimiters: usize,
    ) -> Result<Self, ParseError<'s>> {
        let mut pieces = Vec::new();

        parser.switch_mode(ParseMode::Raw);
        parser.advance();
        loop {
            let tok = parser
                .current_raw()
                .expect("TemplateString::parse called on err token")
                .expect("TemplateString::parse called on no token");
            match tok {
                RawToken::RawPart(s) => {
                    eprint!("'{s}' ");
                    pieces.push(TemplatePiece::StrVal(s));
                    parser.advance();
                }
                RawToken::Newline => {
                    pieces.push(TemplatePiece::Char('\n'));
                    parser.advance();
                }
                RawToken::Escaped(ch) => {
                    match ch {
                        ch @ ('\n' | '\t' | '\\' | '\'' | '[') => {
                            pieces.push(TemplatePiece::Char(ch))
                        }
                        '\r' => {}
                        c => panic!("Unknown escape sequence: {c:?}"),
                    }
                    eprint!("{ch}");
                    parser.advance();
                }
                RawToken::ExprStart => {
                    eprint!(">> ");
                    parser.switch_mode(ParseMode::Expr);
                    parser.advance();
                    pieces.push(TemplatePiece::Expr(Expr::parse(parser, ParseMode::Raw)?));
                }
                RawToken::TemplateStringDelimiter(n) if n == number_delimiters => {
                    eprint!("''' >> ");
                    break;
                }
                RawToken::TemplateStringDelimiter(_) => {
                    eprint!("'{}' ", parser.raw_lexer.slice());
                    pieces.push(TemplatePiece::StrVal(parser.raw_lexer.slice()));
                }
                RawToken::IgnoredLineContinuation => unreachable!(),
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
    Char(char),
    Expr(Expr<'s>),
}

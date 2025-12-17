use crate::{
    errors::{expansion_error::ExpansionResult, parse_error::ParseError},
    expand::Expanded,
    lexer::RawToken,
    parser::ParseMode,
    source_type::{Borrowed, SourceType},
};

use super::*;

#[derive(Clone, Debug)]
pub struct TemplateString<S: SourceType> {
    pub pieces: Vec<TemplatePiece<S>>,
}

impl<S: SourceType> Expandable<S> for TemplateString<S> {
    fn expand(self, ctx: &ProgramContext<S>) -> ExpansionResult {
        let mut result = String::new();
        for piece in self.pieces.into_iter() {
            match piece {
                TemplatePiece::Char(ch) => result.push(ch),
                TemplatePiece::StrVal(s) => result.push_str(s.as_ref()),
                TemplatePiece::Expr(expr) => result.push_str(&expr.expand(ctx)?.into_string()),
            }
        }
        Ok(Expanded::Str(result))
    }
}

impl<'s> TemplateString<Borrowed<'s>> {
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
                    parser.advance();
                }
                RawToken::ExprStart => {
                    parser.switch_mode(ParseMode::Expr);
                    parser.advance();
                    pieces.push(TemplatePiece::Expr(Expr::parse(parser, ParseMode::Raw)?));
                }
                RawToken::TemplateStringDelimiter(n) if n == number_delimiters => {
                    break;
                }
                RawToken::TemplateStringDelimiter(_) => {
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
pub enum TemplatePiece<S: SourceType> {
    StrVal(S::Str),
    Char(char),
    Expr(Expr<S>),
}

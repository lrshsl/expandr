use crate::{
    errors::parse_error::ParseError,
    lexer::RawToken,
    parser::TokenizationMode,
    source_type::{Borrowed, SourceType},
};

use super::*;

#[derive(Clone, Debug)]
pub struct TemplateString<S: SourceType> {
    pub pieces: Vec<TemplatePiece<S>>,
}

impl<'s> TemplateString<Borrowed<'s>> {
    pub fn parse(parser: &mut Parser<'s>, number_delimiters: usize) -> Result<Self, ParseError> {
        let mut pieces = Vec::new();

        parser.switch_mode(TokenizationMode::Raw);
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
                        ch @ ('\n' | '\t' | '\\' | '\'' | '[' | ']') => {
                            pieces.push(TemplatePiece::Char(ch))
                        }
                        '\r' => {}
                        c => panic!("Unknown escape sequence: {c:?} in {:?}", parser.ctx()),
                    }
                    parser.advance();
                }
                RawToken::ExprStart => {
                    parser.switch_mode(TokenizationMode::Expr);
                    parser.advance();
                    pieces.push(TemplatePiece::Expr(Expr::parse(
                        parser,
                        TokenizationMode::Raw,
                    )?));
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
        parser.switch_mode(TokenizationMode::Expr);
        parser.advance();

        Ok(Self { pieces })
    }
}

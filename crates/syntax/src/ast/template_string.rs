use crate::{
    errors::parse_error::ParseError,
    lexer::RawToken,
    parser::TokenizationMode,
    source_type::{Borrowed, SourceType},
    unexpected_eof,
};

use super::*;

#[derive(Clone, Debug)]
pub struct TemplateString<S: SourceType> {
    pub pieces: Vec<TemplatePiece<S>>,
}

impl<'s> TemplateString<Borrowed<'s>> {
    pub fn parse(parser: &mut Parser<'s>, end_token: RawToken) -> Result<Self, ParseError> {
        let mut pieces = Vec::new();

        parser.switch_mode(TokenizationMode::Raw);
        parser.advance();
        loop {
            let Some(tok) = parser
                .current_raw()
                .expect("TemplateString::parse called on err token")
            else {
                unexpected_eof!(parser.ctx())?
            };
            match tok {
                t if t == end_token => {
                    break;
                }
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
                        ch @ ('\n' | '\t' | '\\' | '\'' | '[' | ']' | '}' | '{') => {
                            pieces.push(TemplatePiece::Char(ch))
                        }
                        '\r' => {}
                        c => panic!("Unknown escape sequence: {c:?} in {:?}", parser.ctx()),
                    }
                    parser.advance();
                }
                RawToken::BlockEnd => {
                    // When not expecting a block end (`end_token != BlockEnd`)
                    // Just insert escaped version?
                    pieces.push(TemplatePiece::Char(']'));
                    pieces.push(TemplatePiece::Char(']'));
                }
                RawToken::ExprStart => {
                    parser.switch_mode(TokenizationMode::Expr);
                    parser.advance();
                    pieces.push(TemplatePiece::Expr(Expr::parse(
                        parser,
                        TokenizationMode::Raw,
                    )?));
                }
                RawToken::TemplateStringDelimiter(_) => {
                    pieces.push(TemplatePiece::StrVal(parser.raw_lexer.slice()));
                    parser.advance();
                }
                RawToken::IgnoredLineContinuation => unreachable!(),
            }
        }
        parser.switch_mode(TokenizationMode::Expr);
        parser.advance();

        Ok(Self { pieces })
    }
}

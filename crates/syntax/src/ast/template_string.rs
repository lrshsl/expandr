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
    pub fn parse(parser: &mut Parser<'s>, end_token: RawToken) -> Result<Self, ParseError> {
        let mut pieces = Vec::new();

        parser.switch_mode(TokenizationMode::Raw);
        parser.advance();
        loop {
            match parser
                .current_raw()
                .expect("TemplateString::parse called on errorous token")
                .ok_or_else(|| ParseError::UnexpectedEof {
                    ctx: (parser.ctx()),
                    file: file!(),
                    line: line!(),
                })? {
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
                RawToken::BlockStart => {
                    parser.switch_mode(TokenizationMode::Expr);
                    parser.advance();
                    assert_eq!(parser.current_expr(), Ok(Some(ExprToken::BlockStart)));
                    pieces.push(TemplatePiece::Expr(Block::parse(parser)?.into()));
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
                    pieces.push(TemplatePiece::StrVal(parser.slice()));
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

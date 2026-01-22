use crate::{
    ast::mapping::Mapping,
    errors::parse_error::ParseResult,
    lexer::{ExprToken, RawToken, Token},
    log,
    parser::TokenizationMode,
    program_context::ProgramContext,
    source_type::{Borrowed, SourceType},
    unexpected_token,
};

use super::*;

#[derive(Debug)]
pub struct Ast<S: SourceType> {
    pub exprs: Vec<Expr<S>>,
    pub imports: Vec<Import>,
    pub ctx: ProgramContext<S>,
}

impl<'s> Parsable<'s> for Ast<Borrowed<'s>> {
    fn parse(parser: &mut Parser<'s>) -> ParseResult<'s, Self> {
        let mut ctx = ProgramContext::new();
        let mut imports = Vec::new();
        let mut exprs = Vec::new();

        loop {
            parser.skip_newlines();
            let Some(token) = parser.current_expr().expect("Ast::parse on invalid token") else {
                break;
            };
            log!("Starting on {token:?}");

            match token {
                ExprToken::Import => {
                    imports.push(Import::parse(parser)?);
                }
                ExprToken::Map => {
                    parser.advance();
                    let Some(Token::ExprToken(ExprToken::Ident(name))) = parser.current()? else {
                        panic!("Expecting ident after keyword 'map'");
                    };
                    parser.advance();
                    let mapping = Mapping::parse(parser)?;
                    match ctx.get_mut(name) {
                        Some(slot) => slot.push(mapping),
                        None => {
                            let _ = ctx.insert(name, vec![mapping]);
                        }
                    }
                }
                ExprToken::Symbol('[') => {
                    parser.advance();
                    exprs.push(Expr::parse(parser, TokenizationMode::Expr)?);
                    parser.skip(ExprToken::Symbol(']'), file!(), line!())?;
                }
                ExprToken::String(strval) => {
                    exprs.push(Expr::StrRef(strval));
                    parser.advance()
                }
                ExprToken::BlockStart => {
                    exprs.push(Block::parse(parser)?.into());
                }
                ExprToken::TemplateStringDelimiter(n) => {
                    // Read template string until next sequence of the same number template string delimiters
                    exprs.push(
                        TemplateString::parse(parser, RawToken::TemplateStringDelimiter(n))?.into(),
                    );
                    parser.advance();
                }
                ExprToken::Ident(_) => {
                    exprs.push(Expr::parse(parser, TokenizationMode::Expr)?);
                }
                tok => unexpected_token!(
                    found   : tok,
                    expected: [Import, Map, Symbol('['), String, BlockStart, TemplateStringDelimiter, Ident],
                    @ parser.ctx()
                )?,
            }
        }

        Ok(Self {
            exprs,
            imports,
            ctx,
        })
    }
}

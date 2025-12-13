use crate::{
    errors::{expansion_error::ExpansionError, parse_error::ParseResult},
    expand::Expanded,
    lexer::{ExprToken, Token},
    log,
    parser::ParseMode,
    unexpected_token,
};

use super::*;

#[derive(Debug)]
pub struct Ast<'s> {
    pub exprs: Vec<Expr<'s>>,
    pub ctx: ProgramContext<'s>,
}

impl<'s> Parsable<'s> for Ast<'s> {
    fn parse(parser: &mut Parser<'s>) -> ParseResult<'s, Self> {
        let mut ctx = ProgramContext::new();
        let mut exprs = Vec::new();

        while {
            let tok = parser.current().expect("Ast::parse on invalid token");
            tok.is_some()
        } {
            let token = parser.current_expr().expect("").expect("");
            log!("Ast::parse starting on {token:?}");
            match token {
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
                    exprs.push(Expr::parse(parser, ParseMode::Expr)?);
                    parser.skip(ExprToken::Symbol(']'))?;
                }
                ExprToken::String(strval) => {
                    exprs.push(Expr::StrRef(strval));
                    parser.advance()
                }
                ExprToken::TemplateStringDelimiter(n) => {
                    exprs.push(TemplateString::parse(parser, n)?.into());
                    parser.advance();
                }
                tok => unexpected_token!(
                    found   : tok,
                    expected: [ExprToken::Map, ExprToken::Symbol('['), ExprToken::String(_)],
                    @ parser.ctx()
                )?,
            }
        }

        Ok(Self { exprs, ctx })
    }
}

impl<'s> Ast<'s> {
    pub fn expand(self) -> (String, Vec<ExpansionError>) {
        let pieces = self.exprs.into_iter().map(|e| e.expand(&self.ctx));
        let mut errs = Vec::new();
        let mut out_str = String::new();
        for piece in pieces {
            match piece {
                Ok(Expanded::Str(s)) => out_str.push_str(&s),
                Ok(Expanded::Int(i)) => out_str.push(
                    char::from_u32(i.try_into().expect("Negative number?"))
                        .expect("This isn't a representable unicode character"),
                ),
                Err(e) => errs.push(e),
            }
        }
        (out_str, errs)
    }
}

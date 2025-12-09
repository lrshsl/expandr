use crate::{
    errors::parse_error::ParseResult,
    expand::Expanded,
    lexer::ExprToken,
    log,
    parser::{ParseMode, Token},
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
            eprint!("Starting with {token:?} >> ");
            match token {
                ExprToken::Map => {
                    parser.advance();
                    let Some(Token::Expr(ExprToken::Ident(name))) = parser.current()? else {
                        panic!("Expecting ident after keyword 'map'");
                    };
                    parser.advance();
                    eprint!("Mapping '{name}' >> ");
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
                    parser.skip(Token::Expr(ExprToken::Symbol(']')))?;
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
            eprintln!("Done\n");
        }

        Ok(Self { exprs, ctx })
    }
}

impl<'s> Ast<'s> {
    pub fn expand(self) -> String {
        self.exprs
            .into_iter()
            .map(|expr| match expr.expand(&self.ctx) {
                Expanded::Str(s) => s,
                Expanded::Int(i) => i.to_string(),
            })
            .collect::<Vec<_>>()
            .join("")
    }
}

use crate::{
    context::{EvaluationContext, ProgramContext},
    errors::{expansion_error::ExpansionError, parse_error::ParseResult},
    expand::{Expandable as _, Expanded},
    lexer::{ExprToken, Token},
    log,
    parser::ParseMode,
    source_type::{Borrowed, Owned, SourceType},
    unexpected_token,
};

use super::*;

#[derive(Debug)]
pub struct Ast<S: SourceType> {
    pub exprs: Vec<Expr<S>>,
    pub imports: Vec<PathIdent<S>>,
    pub ctx: ProgramContext<S>,
}

impl<'s> Parsable<'s> for Ast<Borrowed<'s>> {
    fn parse(parser: &mut Parser<'s>) -> ParseResult<'s, Self> {
        let mut ctx = ProgramContext::new();
        let mut imports = Vec::new();
        let mut exprs = Vec::new();

        while {
            let tok = parser.current().expect("Ast::parse on invalid token");
            tok.is_some()
        } {
            let token = parser.current_expr().unwrap().unwrap();
            log!("Ast::parse starting on {token:?}");
            match token {
                ExprToken::Use => {
                    parser.advance();
                    imports.push(PathIdent::parse(parser)?);
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
                    expected: [ExprToken::Use, ExprToken::Map, ExprToken::Symbol('['), ExprToken::String(_)],
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

impl<S: SourceType> Ast<S> {
    /// Imports must be handled already and passed in as argument
    pub fn expand<Ctx: EvaluationContext<Owned>>(
        self,
        imported_ctx: &Ctx,
    ) -> (String, Vec<ExpansionError>) {
        let pieces = self.exprs.into_iter().map(|e| e.expand(imported_ctx));
        let mut errs = Vec::new();
        let mut out_str = String::new();

        // Expand all pieces, joining into string, collecting errors
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

use crate::{errs::ParsingError, lexer::ExprToken, unexpected_token};

use super::*;

#[derive(Debug)]
pub struct Ast<'s> {
    pub mappings: ProgramContext<'s>,
    pub exprs: Vec<Expr<'s>>,
}

impl<'s> Parsable<'s> for Ast<'s> {
    fn parse(parser: &mut Parser<'s>) -> Result<Ast<'s>, ParsingError<'s>> {
        let mut mappings = ProgramContext::new();
        let mut exprs = Vec::new();

        while let Some(token) = parser.current() {
            print!("Starting with {token:?} >> ");
            match token {
                ExprToken::Map => {
                    parser.advance();
                    let Some(ExprToken::Ident(name)) = parser.current() else {
                        panic!("Expecting ident after keyword 'map'");
                    };
                    parser.advance();
                    print!("Mapping '{name}' >> ");
                    let mapping = Mapping::parse(parser)?;
                    match mappings.get_mut(name) {
                        Some(slot) => slot.push(mapping),
                        None => {
                            let _ = mappings.insert(name, vec![mapping]);
                        }
                    }
                }
                ExprToken::Symbol('[') => exprs.push(Expr::parse(parser)?),
                ExprToken::String(strval) => {
                    exprs.push(Expr::String(strval));
                    parser.advance()
                }
                tok => {
                    unexpected_token!(
                        found   : tok,
                        expected: [ExprToken::Map, ExprToken::Symbol('['), ExprToken::String(_)],
                        @ &parser.expr_lexer.extras);
                }
            }
            println!();
        }

        Ok(Self { mappings, exprs })
    }
}

use crate::{
    errs::ParsingError,
    lexer::ExprToken,
    log,
    parser::{ParseMode, Token},
    unexpected_token,
};

use super::*;

#[derive(Debug)]
pub struct Ast<'s> {
    pub ctx: ProgramContext<'s>,
    pub exprs: Vec<Expr<'s>>,
}

impl<'s> Parsable<'s> for Ast<'s> {
    fn parse(parser: &mut Parser<'s>) -> Result<Ast<'s>, ParsingError<'s>> {
        let mut mappings = ProgramContext::new();
        let mut exprs = Vec::new();

        while let Some(Token::Expr(token)) = parser.current() {
            log!("Ast::parse starting on {token:?}");
            eprint!("Starting with {token:?} >> ");
            match token {
                ExprToken::Map => {
                    parser.advance();
                    let Some(Token::Expr(ExprToken::Ident(name))) = parser.current() else {
                        panic!("Expecting ident after keyword 'map'");
                    };
                    parser.advance();
                    eprint!("Mapping '{name}' >> ");
                    let mapping = Mapping::parse(parser)?;
                    match mappings.get_mut(name) {
                        Some(slot) => slot.push(mapping),
                        None => {
                            let _ = mappings.insert(name, vec![mapping]);
                        }
                    }
                }
                ExprToken::Symbol('[') => {
                    parser.advance();
                    exprs.push(Expr::parse(parser, ParseMode::Expr)?);
                    parser.skip(Token::Expr(ExprToken::Symbol(']'))); // ']'
                }
                ExprToken::String(strval) => {
                    exprs.push(Expr::StrRef(strval));
                    parser.advance()
                }
                ExprToken::TemplateStringDelimiter(n) => {
                    exprs.push(Expr::TemplateString(TemplateString::parse(parser, n)?));
                    parser.advance();
                }
                tok => {
                    unexpected_token!(
                        found   : tok,
                        expected: [ExprToken::Map, ExprToken::Symbol('['), ExprToken::String(_)],
                        @ &parser.expr_lexer.extras);
                }
            }
            eprintln!("Done\n");
        }

        Ok(Self {
            ctx: mappings,
            exprs,
        })
    }
}

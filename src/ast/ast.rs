use crate::lexer::Token;

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
                Token::Map => {
                    parser.advance();
                    let Some(Token::Ident(name)) = parser.current() else {
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
                Token::Symbol('[') => exprs.push(Expr::parse(parser)?),
                tok => todo!("{tok:?} in {ctx:?}", ctx = parser.context()),
            }
            println!();
        }

        Ok(Self { mappings, exprs })
    }
}

use crate::lexer::Token;

use super::*;

#[derive(Debug)]
pub struct Ast<'s> {
    pub mappings: HashMap<&'s str, Mapping<'s>>,
    pub exprs: Vec<Expr<'s>>,
}

impl<'s> Parsable<'s> for Ast<'s> {
    fn parse(parser: &mut Parser<'s>) -> Result<Ast<'s>, ParsingError<'s>> {
        let mut mappings = HashMap::new();
        let mut exprs = Vec::new();

        while let Some(token) = parser.next_token() {
            match token {
                Token::Map => {
                    let Some(Token::Ident(name)) = parser.next_token() else {
                        panic!("Abrubt eof");
                        // return ParsingError::AbruptEof(, (), ())
                    };
                    let mapping = Mapping::parse(parser)?;
                    mappings.insert(name, mapping);
                }
                Token::Symbol('[') => exprs.push(Expr::parse(parser)?),
                tok => todo!("{tok:?}"),
            }
        }

        Ok(Self { mappings, exprs })
    }
}

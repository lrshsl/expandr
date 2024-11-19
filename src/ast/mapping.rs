use super::*;

#[derive(Debug)]
pub struct Mapping<'s> {
    pub params: Vec<MappingParam<'s>>,
    pub translation: Expr<'s>,
}

impl<'s> Parsable<'s> for Mapping<'s> {
    fn parse(parser: &mut Parser<'s>) -> Result<Self, ParsingError<'s>>
    where
        Self: Sized,
    {
        let mut params = Vec::new();
        while parser.unpack_token()? != Token::Becomes {
            params.push(MappingParam::parse(parser)?);
        }
        parser.advance();
        let translation = match parser
            .current()
            .ok_or(ParsingError::AbruptEof(parser.lexer.extras.clone()))?
        {
            Token::TemplateString(value) => Expr::String(value),
            tok => panic!("Unexpected token: {tok:?}"),
        };
        Ok(Self {
            params,
            translation,
        })
    }
}

#[derive(Debug)]
pub enum MappingParam<'s> {
    Ident(&'s str),
    ParamExpr {
        name: &'s str,
        rep: Option<Repetition>,
    },
}

impl<'s> Parsable<'s> for MappingParam<'s> {
    fn parse(parser: &mut Parser<'s>) -> Result<Self, ParsingError<'s>>
    where
        Self: Sized,
    {
        match parser.unpack_token()? {
            Token::Ident(value) => {
                parser.advance();
                Ok(Self::Ident(value))
            }
            Token::Symbol('[') => {
                parser.advance();
                let Token::Ident(name) = parser.unpack_token()? else {
                    panic!("Expecting ident");
                };
                parser.advance();
                let rep = match parser.unpack_token()? {
                    Token::Symbol('*') => {
                        parser.advance();
                        Some(Repetition::Any)
                    }
                    Token::Symbol('?') => {
                        parser.advance();
                        Some(Repetition::Optional)
                    }
                    Token::Symbol('{') => {
                        todo!();
                        //let Some(Token::Number)
                        //Some(Repetition::Exactly(1))
                    }
                    Token::Symbol(']') => None,
                    tok => panic!("Unexpected token: {tok:?}"),
                };

                assert_eq!(parser.unpack_token()?, Token::Symbol(']'));
                parser.advance();

                Ok(Self::ParamExpr { name, rep })
            }
            tok => todo!("{tok:?}"),
        }
    }
}

#[derive(Debug)]
pub enum Repetition {
    Exactly(usize),
    Optional,
    Any,
}

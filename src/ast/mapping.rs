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
        loop {
            match parser
                .next_token()
                .ok_or(ParsingError::AbruptEof(parser.lexer.extras.clone()))?
            {
                Token::Becomes => {
                    parser.advance();
                    break;
                }
                Token::Ident(value) => todo!(),
                tok => panic!("Unexpected token: {tok:?}"),
            }
        }
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
    Expr(ParamExpr<'s>),
}

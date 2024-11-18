use super::*;

#[derive(Debug)]
pub struct Mapping<'s> {
    pub args: Vec<MappingParam<'s>>,
    pub translation: Expr<'s>,
}

impl<'s> Parsable<'s> for Mapping<'s> {
    fn parse(parser: &mut Parser<'s>) -> Result<Self, ParsingError<'s>>
    where
        Self: Sized,
    {
        Ok(Self {
            args: vec![],
            translation: Expr::String("todo"),
        })
    }
}

#[derive(Debug)]
pub enum MappingParam<'s> {
    Ident(&'s str),
    Expr(ParamExpr<'s>),
}

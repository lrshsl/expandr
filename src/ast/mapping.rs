use crate::{errs::ParsingError, parser::ParseMode};

use super::*;

#[derive(Clone, Debug)]
pub struct Params<'s> {
    pub entries: Vec<MappingParam<'s>>,
}

impl<'s> Params<'s> {
    pub fn matches_args(&self, other: &Vec<Expr<'_>>) -> bool {
        self.entries.len() == other.len()
            && self
                .entries
                .iter()
                .zip(other.iter())
                .all(|(param, arg)| param.matches_arg(arg))
    }
}

#[derive(Clone, Debug)]
pub struct Mapping<'s> {
    pub params: Params<'s>,
    pub translation: Expr<'s>,
}

impl<'s> Parsable<'s> for Mapping<'s> {
    fn parse(parser: &mut Parser<'s>) -> Result<Self, ParsingError<'s>> {
        eprint!("Params >> ");
        let mut params = Vec::new();
        while parser.current_expr().expect("Unfinished map definition") != ExprToken::Becomes {
            params.push(MappingParam::parse(parser)?);
        }
        parser.advance(); // Skip '=>'
        let translation = match parser.current_expr().expect("Unfinished map definition") {
            ExprToken::String(value) => {
                parser.advance();
                eprint!("Output String({value:?})");
                Expr::StrRef(value)
            }
            ExprToken::TemplateStringDelimiter(n) => {
                eprint!("Output Template String >> ");
                let s = TemplateString::parse(parser, n)?;
                Expr::TemplateString(s)
            }
            ExprToken::Symbol('[') => {
                parser.advance();
                Expr::parse(parser, ParseMode::Expr)?
            }
            tok => panic!("Unexpected token: {tok:?}"),
        };
        Ok(Self {
            params: Params { entries: params },
            translation,
        })
    }
}

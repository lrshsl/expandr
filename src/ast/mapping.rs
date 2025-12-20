use std::fmt;

use crate::{
    errors::parse_error::ParseResult, parser::ParseMode, source_type::Borrowed,
    source_type::SourceType, unexpected_token,
};

use super::*;

#[derive(Clone)]
pub struct Params {
    pub entries: Vec<MappingParam>,
}

impl fmt::Debug for Params {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self.entries)
    }
}

impl Params {
    pub fn matches_args<S: SourceType>(&self, other: &Vec<Expr<S>>) -> bool {
        // TODO: convert and compare as Borrowed version
        self.entries.len() == other.len()
            && self
                .entries
                .iter()
                .zip(other.iter())
                .all(|(param, arg)| param.matches_arg(arg))
    }
}

#[derive(Debug, Clone)]
pub enum Mapping<S: SourceType> {
    SimpleMapping(Expr<S>),
    ParameterizedMapping(ParameterizedMapping<S>),
}

#[derive(Clone)]
pub struct ParameterizedMapping<S: SourceType> {
    pub params: Params,
    pub translation: Expr<S>,
}

impl<S: SourceType> fmt::Debug for ParameterizedMapping<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "map {:#?} => {:#?}", self.params, self.translation)
    }
}

impl<'s> Parsable<'s> for Mapping<Borrowed<'s>> {
    fn parse(parser: &mut Parser<'s>) -> ParseResult<'s, Self> {
        let mut params = Vec::new();

        // Params
        while parser.current_expr()?.expect("Unfinished map definition") != ExprToken::Becomes {
            params.push(MappingParam::parse(parser)?);
        }
        parser.skip(ExprToken::Becomes, file!(), line!())?;

        // Translation
        let translation = match parser.current_expr()?.expect("Unfinished map definition") {
            ExprToken::String(value) => {
                parser.advance();
                Ok(Expr::StrRef(value))
            }
            ExprToken::TemplateStringDelimiter(n) => {
                let s = TemplateString::parse(parser, n)?;
                Ok(Expr::TemplateString(s))
            }
            ExprToken::Symbol('.') => PathIdent::parse(parser).map(Into::into),
            ExprToken::Symbol('[') => {
                parser.advance();
                let expr = Expr::parse(parser, ParseMode::Expr);
                parser.skip(ExprToken::Symbol(']'), file!(), line!())?;
                expr
            }
            tok => {
                unexpected_token!(
                    found: tok,
                    expected: [
                        String(_), TemplateStringDelimiter(_),
                        Symbol('[')],
                    @ parser.ctx())
            }
        }?;
        Ok(if params.is_empty() {
            Self::SimpleMapping(translation)
        } else {
            Self::ParameterizedMapping(ParameterizedMapping {
                params: Params { entries: params },
                translation,
            })
        })
    }
}

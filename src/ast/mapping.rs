use crate::{
    errors::parse_error::ParseResult, parser::ParseMode, source_type::Borrowed,
    source_type::SourceType, unexpected_token,
};

use super::*;

#[derive(Clone, Debug)]
pub struct Params<S: SourceType> {
    pub entries: Vec<MappingParam<S>>,
}

impl<S: SourceType> Params<S> {
    pub fn matches_args(&self, other: &Vec<Expr<S>>) -> bool {
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
    Simple(Expr<S>),
    Parameterized(ParameterizedMapping<S>),
}

#[derive(Debug, Clone)]
pub struct ParameterizedMapping<S: SourceType> {
    pub params: Params<S>,
    pub translation: Expr<S>,
}

impl<'s> Parsable<'s> for Mapping<Borrowed<'s>> {
    fn parse(parser: &mut Parser<'s>) -> ParseResult<'s, Self> {
        let mut params = Vec::new();

        while parser.current_expr()?.expect("Unfinished map definition") != ExprToken::Becomes {
            params.push(MappingParam::parse(parser)?);
        }
        parser.skip(ExprToken::Becomes, file!(), line!())?;
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
            Self::Simple(translation)
        } else {
            Self::Parameterized(ParameterizedMapping {
                params: Params { entries: params },
                translation,
            })
        })
    }
}

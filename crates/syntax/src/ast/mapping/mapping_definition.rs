use std::fmt;

use super::{param::Param, params::Params};
use crate::{
    ast::{Expr, TemplateString},
    errors::parse_error::ParseResult,
    lexer::ExprToken,
    parser::{Parsable, Parser, TokenizationMode},
    source_type::{Borrowed, SourceType},
    unexpected_token,
};

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
            params.push(Param::parse(parser)?);
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
            ExprToken::Symbol('[') => {
                parser.advance();
                let expr = Expr::parse(parser, TokenizationMode::Expr);
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

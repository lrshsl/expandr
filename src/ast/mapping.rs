use crate::{
    errs::ParsingError,
    parser::{ParseMode, Token},
    unexpected_token,
};

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

#[derive(Debug, Clone)]
pub enum Mapping<'s> {
    Simple(Expr<'s>),
    Parameterized(ParameterizedMapping<'s>),
}

#[derive(Debug, Clone)]
pub struct ParameterizedMapping<'s> {
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
        parser.skip(Token::Expr(ExprToken::Becomes));
        let translation = match parser.current_expr().expect("Unfinished map definition") {
            ExprToken::String(value) => {
                parser.advance();
                eprint!("Output String({value:?})");
                Ok(Expr::StrRef(value))
            }
            ExprToken::TemplateStringDelimiter(n) => {
                eprint!("Output Template String >> ");
                let s = TemplateString::parse(parser, n)?;
                Ok(Expr::TemplateString(s))
            }
            ExprToken::Symbol('[') => {
                parser.advance();
                let expr = Expr::parse(parser, ParseMode::Expr);
                parser.skip(Token::Expr(ExprToken::Symbol(']'))); // ']'
                expr
            }
            tok => {
                unexpected_token!(
                    found: tok,
                    expected: [
                        String(_), TemplateStringDelimiter(_),
                        Symbol('(' | '[')],
                    @ &parser.expr_lexer.extras);
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

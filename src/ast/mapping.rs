use crate::{errs::ParsingError, unexpected_eof, unexpected_token};

use super::*;

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Mapping<'s> {
    pub params: Params<'s>,
    pub translation: Expr<'s>,
}

impl<'s> Parsable<'s> for Mapping<'s> {
    fn parse(parser: &mut Parser<'s>) -> Result<Self, ParsingError<'s>>
    where
        Self: Sized,
    {
        let mut params = Vec::new();
        while parser.current_expr().expect("Unfinished map definition") != ExprToken::Becomes {
            params.push(MappingParam::parse(parser)?);
        }
        print!("Params {params:?} >> ");
        parser.advance(); // Skip '=>'
        let translation = match parser.current_expr().expect("Unfinished map definition") {
            ExprToken::String(value) => {
                parser.advance();
                print!("Output String({value:?})");
                Expr::String(value)
            }
            ExprToken::TemplateStringDelimiter(n) => {
                let s = TemplateString::parse(parser, n)?;
                print!("Output {s:?} >> ");
                Expr::TemplateString(s)
            }
            ExprToken::Symbol('[') => {
                parser.advance();
                Expr::parse(parser)?
            }
            tok => panic!("Unexpected token: {tok:?}"),
        };
        Ok(Self {
            params: Params { entries: params },
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

impl MappingParam<'_> {
    fn matches_arg(&self, arg: &Expr<'_>) -> bool {
        match (self, arg) {
            (Self::ParamExpr { .. }, Expr::String(_) | Expr::MappingApplication { .. }) => true,
            (Self::Ident(self_value), Expr::Ident(other_value)) => self_value == other_value,
            _ => false,
        }
    }
}

impl<'s> Parsable<'s> for MappingParam<'s> {
    fn parse(parser: &mut Parser<'s>) -> Result<Self, ParsingError<'s>>
    where
        Self: Sized,
    {
        match parser
            .current_expr()
            .expect("MappingParam::parse on no token")
        {
            ExprToken::Ident(value) => {
                parser.advance();
                Ok(Self::Ident(value))
            }
            ExprToken::Symbol('[') => {
                parser.advance();
                let ExprToken::Ident(name) = parser.current_expr().expect("Expected ident") else {
                    panic!("Expecting ident");
                };
                parser.advance();
                let rep = match parser.current_expr() {
                    Some(ExprToken::Symbol('*')) => {
                        parser.advance();
                        Some(Repetition::Any)
                    }
                    Some(ExprToken::Symbol('?')) => {
                        parser.advance();
                        Some(Repetition::Optional)
                    }
                    Some(ExprToken::Symbol('{')) => {
                        todo!();
                        //let Some(Token::Number)
                        //Some(Repetition::Exactly(1))
                    }
                    Some(ExprToken::Symbol(']')) => None,
                    None => unexpected_eof!(&parser.expr_lexer.extras),
                    tok => {
                        unexpected_token!(
                                found: tok,
                                expected: [ExprToken::Symbol('*' | '?' | '{' | ']')],
                                @&parser.expr_lexer.extras
                        );
                    }
                };

                assert_eq!(parser.current_expr(), Some(ExprToken::Symbol(']')));
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

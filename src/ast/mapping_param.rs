use crate::{
    ast::{Expr, ExprToken, Parsable, Parser},
    errors::parse_error::ParseResult,
    unexpected_eof, unexpected_token,
};

#[derive(Clone, Debug)]
pub enum MappingParam<'s> {
    Ident(&'s str),
    ParamExpr {
        name: &'s str,
        rep: Option<Repetition>,
    },
    Symbol(char),
}

impl MappingParam<'_> {
    pub fn matches_arg(&self, arg: &Expr<'_>) -> bool {
        match (self, arg) {
            (
                Self::ParamExpr { .. },
                Expr::Integer(_)
                | Expr::String(_)
                | Expr::TemplateString(_)
                | Expr::MappingApplication { .. },
            ) => true,
            (Self::Ident(self_value), Expr::Ident(other_value)) => self_value == other_value,
            (Self::Symbol(self_value), Expr::LiteralSymbol(other_value)) => {
                self_value == other_value
            }
            _ => false,
        }
    }
}

impl<'s> Parsable<'s> for MappingParam<'s> {
    fn parse(parser: &mut Parser<'s>) -> ParseResult<'s, Self> {
        match parser
            .current_expr()?
            .expect("MappingParam::parse on no token")
        {
            ExprToken::Ident(value) => {
                parser.advance();
                Ok(Self::Ident(value))
            }
            ExprToken::Symbol('[') => {
                parser.advance();
                let ExprToken::Ident(name) = parser.current_expr()?.expect("Expected ident") else {
                    panic!("Expecting ident");
                };
                parser.advance();
                let rep = match parser.current_expr()? {
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
                    None => unexpected_eof!(parser.ctx())?,
                    tok => unexpected_token!(
                            found: tok,
                            expected: [ExprToken::Symbol('*' | '?' | '{' | ']')],
                            @ parser.ctx()
                    )?,
                };

                parser.skip(ExprToken::Symbol(']'))?;

                Ok(Self::ParamExpr { name, rep })
            }
            ExprToken::Symbol(s) if s != '[' => {
                parser.advance();
                Ok(Self::Symbol(s))
            }
            tok => unexpected_token!(
                found: tok,
                expected: [Ident, Expr, Symbol],
                @ parser.ctx()
            )?,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Repetition {
    Exactly(usize),
    Optional,
    Any,
}

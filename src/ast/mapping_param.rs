use crate::{
    ast::{Expr, ExprToken, Parsable, Parser},
    errs::ParsingError,
    unexpected_eof, unexpected_token,
};

#[derive(Clone, Debug)]
pub enum MappingParam<'s> {
    Ident(&'s str),
    ParamExpr {
        name: &'s str,
        rep: Option<Repetition>,
    },
}

impl MappingParam<'_> {
    pub fn matches_arg(&self, arg: &Expr<'_>) -> bool {
        match (self, arg) {
            (
                Self::ParamExpr { .. },
                Expr::String(_) | Expr::TemplateString(_) | Expr::MappingApplication { .. },
            ) => true,
            (Self::Ident(self_value), Expr::Ident(other_value)) => self_value == other_value,
            _ => false,
        }
    }
}

impl<'s> Parsable<'s> for MappingParam<'s> {
    fn parse(parser: &mut Parser<'s>) -> Result<Self, ParsingError<'s>> {
        match parser
            .current_expr()
            .expect("MappingParam::parse on no token")
        {
            ExprToken::Ident(value) => {
                eprint!("Ident({:?}) >> ", value);
                parser.advance();
                Ok(Self::Ident(value))
            }
            ExprToken::Symbol('[') => {
                parser.advance();
                let ExprToken::Ident(name) = parser.current_expr().expect("Expected ident") else {
                    panic!("Expecting ident");
                };
                eprint!("ParamExpr({:?}) >> ", name);
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

#[derive(Clone, Debug)]
pub enum Repetition {
    Exactly(usize),
    Optional,
    Any,
}

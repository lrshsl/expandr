use crate::{
    ast::{Expr, ExprToken, Parsable, Parser, PathIdent},
    errors::parse_error::ParseResult,
    source_type::{Borrowed, SourceType},
    unexpected_eof, unexpected_token,
};

#[derive(Clone, Copy, Debug)]
pub enum ParamType {
    Expr,
    Ident,
}

#[derive(Clone)]
pub enum MappingParam<S: SourceType> {
    Ident(PathIdent),
    ParamExpr {
        name: S::Str,
        rep: Option<Repetition>,
        typ: ParamType,
    },
    Symbol(char),
}

impl<S: SourceType> std::fmt::Debug for MappingParam<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParamExpr {
                name,
                typ,
                rep: Some(rep),
            } => {
                write!(f, "Expr(name: {:?}, type: {:?}, rep: {rep:?})", name, typ)
            }
            Self::ParamExpr { name, typ, .. } => {
                write!(f, "Expr(name: {:?}, type: {:?})", name, typ)
            }
            Self::Ident(ident) => write!(f, "Exactly('{:?}')", ident),
            Self::Symbol(c) => write!(f, "Exactly('{}')", c),
        }
    }
}

impl<S: SourceType> MappingParam<S> {
    pub fn matches_arg(&self, arg: &Expr<S>) -> bool {
        match (self, arg) {
            (
                Self::ParamExpr {
                    typ: ParamType::Expr,
                    ..
                },
                Expr::Integer(_)
                | Expr::String(_)
                | Expr::StrRef(_)
                | Expr::TemplateString(_)
                | Expr::MappingApplication { .. }
                | Expr::PathIdent(_),
            ) => true,

            (
                Self::ParamExpr {
                    typ: ParamType::Ident,
                    ..
                },
                Expr::PathIdent(_),
            ) => true,

            // Raw literal matches
            (Self::Ident(self_value), Expr::PathIdent(other_value)) => self_value == other_value,
            (Self::Symbol(self_value), Expr::LiteralSymbol(other_value)) => {
                self_value == other_value
            }

            _ => false,
        }
    }
}

impl<'s> Parsable<'s> for MappingParam<Borrowed<'s>> {
    fn parse(parser: &mut Parser<'s>) -> ParseResult<'s, Self> {
        match parser
            .current_expr()?
            .expect("MappingParam::parse on no token")
        {
            ExprToken::Ident(value) => {
                parser.advance();
                Ok(Self::Ident(PathIdent::from_str(value)))
            }
            ExprToken::Symbol('[') => {
                parser.advance();
                let ExprToken::Ident(name) = parser.current_expr()?.expect("Expected ident") else {
                    unexpected_token!(found: parser.current_expr()?, expected: [Ident], @ parser.ctx())?
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
                    Some(ExprToken::Symbol(']' | ':')) => None,
                    None => unexpected_eof!(parser.ctx())?,
                    tok => unexpected_token!(
                            found: tok,
                            expected: [ExprToken::Symbol('*' | '?' | '{' | ']')],
                            @ parser.ctx()
                    )?,
                };

                // Optionally a type (`[a:ident]`, `[a:expr]`)
                let typ = if parser.current_expr()? == Some(ExprToken::Symbol(':')) {
                    parser.advance(); // ':'
                    let typ = match parser.current_expr()? {
                        Some(ExprToken::Ident("ident")) => ParamType::Ident,
                        Some(ExprToken::Ident("expr")) => ParamType::Expr,
                        Some(ExprToken::Ident(ident)) => {
                            panic!("Unknown param type specifier: {ident}")
                        }
                        tok => unexpected_token!(
                            found: tok,
                            expected: [ExprToken::Ident],
                            @ parser.ctx()
                        )?,
                    };
                    parser.advance();
                    typ
                } else {
                    ParamType::Expr
                };

                parser.skip(ExprToken::Symbol(']'), file!(), line!())?;

                Ok(Self::ParamExpr { name, rep, typ })
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

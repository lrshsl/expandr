use std::collections::HashMap;

pub(self) use crate::parser::{Parsable, Parser, ParsingError};
pub(self) use crate::{expand::Expandable, lexer::Token};

mod ast;
pub use ast::Ast;

mod expr;
pub use expr::Expr;

mod mapping;
pub use mapping::Mapping;

#[derive(Debug)]
pub struct ParamExpr<'s> {
    expr: Expr<'s>,
    number_repetitions: Repetition,
}

#[derive(Debug)]
pub enum Repetition {
    Exactly(usize),
    Optional,
    Any,
}

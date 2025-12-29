use std::fmt;

use crate::{ast::Expr, source_type::SourceType};

#[derive(Clone)]
pub enum TemplatePiece<S: SourceType> {
    StrVal(S::Str),
    Char(char),
    Expr(Expr<S>),
}

impl<S: SourceType> fmt::Debug for TemplatePiece<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Expr(expr) => write!(f, "Expr({expr:?})"),
            TemplatePiece::StrVal(s) => write!(f, "{s:?}"),
            TemplatePiece::Char(ch) => write!(f, "{ch:?}"),
        }
    }
}

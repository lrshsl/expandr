use crate::{
    lexer::ExprToken,
    parser::{Parsable, Parser},
};

#[macro_export]
macro_rules! derive_from {
    ($src:ident for $dst:ty) => {
        impl From<$src> for $dst {
            fn from(s: $src) -> Self {
                <$dst>::$src(s)
            }
        }
    };
    ($src:ident for $dst:ident where $t:ident : $bound:ident) => {
        impl<$t: $bound> From<$src<$t>> for $dst<$t> {
            fn from(s: $src<$t>) -> Self {
                <$dst<$t>>::$src(s)
            }
        }
    };
    ($src:ident for $dst:ty, lt<$lt:lifetime>) => {
        impl<$lt> From<$src<$lt>> for $dst {
            fn from(s: $src<$lt>) -> Self {
                <$dst>::$src(s)
            }
        }
    };
}

mod ast;
pub use ast::Ast;

mod import;
pub use import::Import;

mod path_ident;
pub use path_ident::{PathIdent, PathIdentRoot};

mod expr;
pub use expr::Expr;

mod template_string;
pub use template_string::TemplateString;

mod template_piece;
pub use template_piece::TemplatePiece;

pub mod mapping;

mod block;
pub use block::Block;

// Used by all files in this module
pub(self) use crate::{
    context::EvaluationContext, expand::Expandable, expansion_error::ExpansionResult,
};
pub(self) use expandr_syntax::source_type::{Owned, SourceType};

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
mod block;
mod expr;
mod is_expr;
mod mapping_application;
mod template_string;

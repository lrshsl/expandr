pub(self) use crate::{
    expand::{Expandable, ProgramContext},
    lexer::ExprToken,
    parser::{Parsable, Parser},
};

#[macro_export]
macro_rules! derive_from {
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

mod expr;
pub use expr::Expr;

mod is_expr;
pub use is_expr::IsExpr;

mod template_string;
pub use template_string::TemplateString;

mod mapping;
pub use mapping::Mapping;

mod mapping_param;
pub use mapping_param::MappingParam;

mod mapping_application;
pub use mapping_application::{Args, MappingApplication};

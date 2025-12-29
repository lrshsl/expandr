use expandr_syntax::ast::Ast;

use super::*;

use crate::{
    context::EvaluationContext,
    expand::{Expandable, Expanded},
    expansion_error::ExpansionResult,
};

impl<S: SourceType> Expandable for Ast<S> {
    /// Imports must be handled already and passed in as argument
    fn expand<Ctx: EvaluationContext<Owned>>(self, imported_ctx: &Ctx) -> ExpansionResult {
        let pieces = self.exprs.into_iter().map(|e| e.expand(imported_ctx));
        let mut out_str = String::new();

        // Expand all pieces, joining into string, collecting errors
        for piece in pieces {
            match piece? {
                Expanded::Str(s) => out_str.push_str(&s),
                Expanded::Int(i) => out_str.push(
                    char::from_u32(i.try_into().expect("Negative number?"))
                        .expect("This isn't a representable unicode character"),
                ),
            }
        }
        Ok(Expanded::Str(out_str))
    }
}

use expandr_syntax::ast::{mapping::MappingApplication, Expr};

use super::*;

impl<S: SourceType> Expandable for Expr<S> {
    fn expand<Ctx: EvaluationContext<Owned>>(self, ctx: &Ctx) -> ExpansionResult {
        use crate::expand::Expanded as E;

        match self {
            Expr::String(val) => Ok(E::Str(val)),
            Expr::StrRef(val) => Ok(E::Str(val.to_string())),

            Expr::TemplateString(tmpl_string) => tmpl_string.expand(ctx),
            Expr::Integer(val) => Ok(E::Int(val)),

            Expr::PathIdent(ident) => {
                // This branch is called when an argument is an Ident although the mapping expects
                // an expression. The ident is treated as an expression in this case (mapping
                // application without arguments), to allow writing things like `[m x + y]` instead
                // of having to write `[m [x] + [y]]` explicitly.
                let pseudo_mapping: MappingApplication<S> = MappingApplication {
                    name: ident,
                    args: vec![],
                };
                pseudo_mapping.expand(ctx)
            }
            Expr::LiteralSymbol(s) => {
                unreachable!("Should not try to expand a literal symbol: {s}")
            }

            Expr::Block(block) => block.expand(ctx),
            Expr::MappingApplication(mapping_application) => mapping_application.expand(ctx),
        }
    }
}

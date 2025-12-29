use expandr_syntax::ast::{Branch, IsExpr, MatchExpr};

use super::*;

impl<S: SourceType> Expandable for IsExpr<S> {
    fn expand<Ctx: EvaluationContext<Owned>>(self, ctx: &Ctx) -> ExpansionResult {
        let cond = self.cond_expr.expand(ctx)?;
        self.branches
            .into_iter()
            .find_map(
                |Branch {
                     match_expr,
                     translation,
                 }| {
                    match match_expr {
                        MatchExpr::MatchAll => Some(translation.expand(ctx)),
                        MatchExpr::Expr(expr) => match expr.expand(ctx) {
                            Ok(res) if cond == res => Some(translation.expand(ctx)),
                            Err(e) => Some(Err(e)),
                            Ok(_) => None,
                        },
                    }
                },
            )
            .expect("No branch matched!")
    }
}

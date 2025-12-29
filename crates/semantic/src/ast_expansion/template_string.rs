use expandr_syntax::ast::{Expr, TemplatePiece, TemplateString};

use crate::expand::Expanded;

use super::*;

impl<S: SourceType> Expandable for TemplateString<S> {
    fn expand<Ctx: EvaluationContext<Owned>>(self, ctx: &Ctx) -> ExpansionResult {
        let mut result = String::new();
        for piece in self.pieces.into_iter() {
            match piece {
                TemplatePiece::Char(ch) => result.push(ch),
                TemplatePiece::StrVal(s) => result.push_str(s.as_ref()),
                TemplatePiece::Expr(Expr::PathIdent(id)) => result.push_str(&id.to_string()),
                TemplatePiece::Expr(expr) => result.push_str(&expr.expand(ctx)?.into_string()),
            }
        }
        Ok(Expanded::Str(result))
    }
}

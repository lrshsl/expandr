use expandr_syntax::ast::Block;

use crate::expand::Expanded;

use super::*;

impl<S: SourceType> Expandable for Block<S> {
    fn expand<Ctx: EvaluationContext<Owned>>(self, ctx: &Ctx) -> ExpansionResult {
        let mut result = String::new();

        for expr in self.exprs {
            result.push_str(&expr.expand(ctx)?.into_string());
            result.push('\n');
        }

        Ok(Expanded::Str(result))
    }
}


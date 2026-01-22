use expandr_syntax::{
    ast::mapping::Args,
    source_type::{Owned, SourceType},
};

use crate::{context::EvaluationContext, expansion_error::ExpansionResult};

mod calculate;
mod is_expr;

type BuiltinFn<S, Ctx> = fn(&Ctx, &Args<S>) -> ExpansionResult;

pub fn get_builtin<S: SourceType, Ctx: EvaluationContext<Owned>>(
    name: &str,
) -> Option<BuiltinFn<S, Ctx>> {
    match name {
        "calc" => Some(calculate::evaluate_math),
        "is" => Some(is_expr::is_expr),
        &_ => None,
    }
}

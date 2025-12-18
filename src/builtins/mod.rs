use crate::{
    ast::Args,
    context::EvaluationContext,
    errors::expansion_error::ExpansionResult,
    source_type::{Owned, SourceType},
};

type BuiltinFn<S, Ctx> = fn(&Ctx, &Args<S>) -> ExpansionResult;

pub fn get_builtin<S: SourceType, Ctx: EvaluationContext<Owned>>(
    name: &str,
) -> Option<BuiltinFn<S, Ctx>> {
    match name {
        "m" => Some(builtin_implementations::evaluate_math),
        &_ => None,
    }
}

mod builtin_implementations {
    use crate::{
        ast::{Args, Expr},
        context::EvaluationContext,
        errors::expansion_error::ExpansionResult,
        expand::{Expandable as _, Expanded},
        source_type::{Owned, SourceType},
    };

    pub fn evaluate_math<S: SourceType, Ctx: EvaluationContext<Owned>>(
        ctx: &Ctx,
        args: &Args<S>,
    ) -> ExpansionResult {
        Ok(match &args[..] {
            [a, Expr::LiteralSymbol('+'), b] => {
                match (a.clone().expand(ctx)?, b.clone().expand(ctx)?) {
                    (Expanded::Int(a), Expanded::Int(b)) => Expanded::Int(a + b),
                    (Expanded::Str(a), Expanded::Str(b)) => Expanded::Str(a + &b),
                    _ => panic!("Operation '+' not defined for Int and String"),
                }
            }
            [a, Expr::LiteralSymbol('*'), b] => {
                match (a.clone().expand(ctx)?, b.clone().expand(ctx)?) {
                    (Expanded::Int(a), Expanded::Int(b)) => Expanded::Int(a * b),
                    (Expanded::Str(s), Expanded::Int(n)) | (Expanded::Int(n), Expanded::Str(s)) => {
                        Expanded::Str(
                            s.repeat(
                                n.try_into()
                                    .expect("Cannot multiply String by negative number"),
                            ),
                        )
                    }
                    _ => panic!("Operation '*' not defined for String and String"),
                }
            }
            _ => Expanded::Str("Error".to_string()),
        })
    }
}

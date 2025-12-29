use expandr_syntax::{
    ast::mapping::Args,
    source_type::{Owned, SourceType},
};

use crate::{context::EvaluationContext, expansion_error::ExpansionResult};

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
    use std::ops;

    use crate::{
        context::EvaluationContext,
        expand::{Expandable as _, Expanded},
        expansion_error::ExpansionResult,
    };
    use expandr_syntax::{
        ast::{mapping::Args, Expr},
        source_type::{Owned, SourceType},
    };

    pub fn evaluate_math<S: SourceType, Ctx: EvaluationContext<Owned>>(
        ctx: &Ctx,
        args: &Args<S>,
    ) -> ExpansionResult {
        let op = match args.get(1) {
            Some(Expr::LiteralSymbol('+')) => <i64 as ops::Add>::add,
            Some(Expr::LiteralSymbol('-')) => ops::Sub::sub,
            Some(Expr::LiteralSymbol('*')) => ops::Mul::mul,
            Some(Expr::LiteralSymbol('/')) => ops::Div::div,
            _ => todo!("No such operation {args:?}"),
        };
        Ok(match &args[..] {
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
            [a, Expr::LiteralSymbol('+' | '-' | '/'), b] => {
                match (a.clone().expand(ctx)?, b.clone().expand(ctx)?) {
                    (Expanded::Int(a), Expanded::Int(b)) => Expanded::Int(op(a, b)),
                    _ => panic!("Operation {:?} only implemented for integers", args[1]),
                }
            }
            _ => Expanded::Str("Error".to_string()),
        })
    }
}

use crate::{
    ast::Args, errors::expansion_error::ExpansionResult, expand::ProgramContext,
    source_type::SourceType,
};

type BuiltinFn<S> = fn(&ProgramContext<S>, &Args<S>) -> ExpansionResult;

pub fn get_builtin<S: SourceType>(name: &str) -> Option<BuiltinFn<S>> {
    Some(match name {
        "m" => builtin_implementations::evaluate_math,
        &_ => return None,
    })
}

mod builtin_implementations {
    use crate::{
        ast::{Args, Expr},
        errors::expansion_error::ExpansionResult,
        expand::{Expandable as _, Expanded, ProgramContext},
        source_type::SourceType,
    };

    pub fn evaluate_math<S: SourceType>(
        ctx: &ProgramContext<S>,
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

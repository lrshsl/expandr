use crate::{
    context::EvaluationContext,
    expand::{Expandable as _, Expanded},
    expansion_error::ExpansionResult,
};
use expandr_syntax::{
    ast::{mapping::Args, Expr},
    source_type::{Owned, SourceType},
};

macro_rules! skip_arg {
    ($args:expr, $pat:pat) => {
        assert!(matches!($args.next(), Some($pat)))
    };
}

pub fn is_expr<S: SourceType, Ctx: EvaluationContext<Owned>>(
    ctx: &Ctx,
    args: &Args<S>,
) -> ExpansionResult {
    let mut args = args.into_iter();

    // Condition
    let Some(expr) = args.next() else {
        panic!("Invalid is expr: no name");
    };
    let condition = expr.clone().expand(ctx)?;

    skip_arg!(args, Expr::LiteralSymbol('{'));

    // Branches
    while let Some(b_cond) = args.next() {
        // Done?
        if matches!(b_cond, Expr::LiteralSymbol('}')) {
            break;
        };

        // Branching pattern that matches
        if pattern_matches_expanded(&condition, &b_cond.clone().expand(ctx)?) {
            skip_arg!(args, Expr::LiteralSymbol('?'));

            let translation = args
                .next()
                .expect("Pattern without translation")
                .clone()
                .expand(ctx);

            // Return translation
            return translation;
        }

        // Skip this branch
        skip_arg!(args, Expr::LiteralSymbol('?'));
        skip_arg!(args, _);
        skip_arg!(args, Expr::LiteralSymbol(','));
    }

    // Expand to nothing if no branch matched
    Ok(Expanded::Str(String::new()))
}

fn pattern_matches_expanded(expr: &Expanded, pattern: &Expanded) -> bool {
    match (expr, pattern) {
        (Expanded::Str(expr_str), Expanded::Str(pattern_str)) => expr_str == pattern_str,
        _ => false,
    }
}

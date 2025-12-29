use expandr_syntax::ast::mapping::{Args, Params};

use super::*;

pub fn matches_args<S: SourceType>(params: &Params, args: &Args<S>) -> bool {
    // TODO: convert and compare as Borrowed version
    params.entries.len() == args.len()
        && params
            .entries
            .iter()
            .zip(args.iter())
            .all(|(param, arg)| param::matches_arg(param, arg))
}

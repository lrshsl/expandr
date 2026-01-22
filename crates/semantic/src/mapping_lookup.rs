use expandr_syntax::{
    ast::{
        mapping::{Mapping, MappingApplication, Param, ParamType, ParameterizedMapping, Params},
        Expr, PathIdent,
    },
    log,
    source_type::{Owned, SourceType},
    ProgramContext,
};

use crate::context::{EvaluationContext, ScopedContext};

// ProgramContext: Global scope
impl<S: SourceType> EvaluationContext<S> for ProgramContext<S>
where
    S::Str: std::borrow::Borrow<str>,
{
    // TODO: Return `MatchingNotFound` error on fail
    fn lookup(&self, path_ident: &PathIdent, args: &[Expr<Owned>]) -> Option<&Mapping<S>> {
        let formatted_mapping = format!("Name: {path_ident}\nArgs: {args:#?}");

        // At least an entry for this name?
        let Some(name_matches) = &self.get(path_ident.name()) else {
            log!("No name matching found for {path_ident} in ProgramContext, there might have been matchings in subscopes, but their arguments didn't match.\n\n{formatted_mapping}");
            return None;
        };

        // Filter the name matches. Only retain the ones where the arguments match the expected
        // parameters
        let mut arg_matches = name_matches
            .iter()
            .filter(|m| mapping_matches_args(m, args));
        // Clone the iterator for printing later
        let arg_matches_copy = arg_matches.clone();

        // At least one full match?
        let Some(first_arg_match) = arg_matches.next() else {
            log!("Found some name matchings for {path_ident}, but arguments didn't match.\n\n{formatted_mapping}\n\nCandidate(s): {name_matches:#?}");
            return None;
        };

        // TODO: Precedence for raw idents vs exprs
        Some(first_arg_match)
    }
}

// ScopedContext: Local scopes
impl<'parent, S: SourceType> EvaluationContext<S> for ScopedContext<'parent, S> {
    fn lookup(&self, path_ident: &PathIdent, args: &[Expr<Owned>]) -> Option<&Mapping<S>> {
        // Try lookup locally first
        if let Some(name_matches) = self.locals.get(path_ident.name()) {
            let mut arg_matches = name_matches
                .iter()
                .filter(|&m| mapping_matches_args(m, args));
            return arg_matches.next();
        }
        // Delegate lookup to parent
        self.parent.lookup(path_ident, args)
    }
}

fn mapping_matches_args<S: SourceType>(mapping: &Mapping<S>, args: &[Expr<Owned>]) -> bool {
    match mapping {
        Mapping::SimpleMapping(_) => args.is_empty(),
        Mapping::ParameterizedMapping(ParameterizedMapping { params, .. }) => {
            matches_args(params, args)
        }
    }
}

fn matches_args<S: SourceType>(params: &Params, args: &[Expr<S>]) -> bool {
    params.entries.len() == args.len()
        && params
            .entries
            .iter()
            .zip(args.iter())
            .all(|(param, arg)| matches_arg(param, arg))
}

fn matches_arg<S: SourceType>(param: &Param, arg: &Expr<S>) -> bool {
    match (param, arg) {
        // Evaluated expressions
        (
            Param::ParamExpr {
                typ: ParamType::Expr,
                ..
            },
            Expr::Integer(_)
            | Expr::String(_)
            | Expr::StrRef(_)
            | Expr::TemplateString(_)
            | Expr::MappingApplication(_)
            | Expr::PathIdent(_)
            | Expr::Block(_),
        ) => true,

        // Idents
        (
            Param::ParamExpr {
                typ: ParamType::Ident,
                ..
            },
            Expr::PathIdent(_),
        ) => true,
        (
            Param::ParamExpr {
                typ: ParamType::Ident,
                ..
            },
            Expr::MappingApplication(appl),
        ) if appl.args.is_empty() => true, // In Expr::parse, idents are also parsed as mapping
        // applications

        // Raw literal matches
        (Param::Ident(self_value), Expr::PathIdent(other_value)) => self_value == other_value,
        (Param::Ident(self_value), Expr::MappingApplication(MappingApplication { name, args }))
            if args.is_empty() =>
        {
            self_value == name
        }
        (Param::Symbol(self_value), Expr::LiteralSymbol(other_value)) => self_value == other_value,

        _ => false,
    }
}

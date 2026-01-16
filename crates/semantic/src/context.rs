use std::collections::HashMap;

use expandr_syntax::{
    ast::{mapping::Mapping, Expr, PathIdent},
    source_type::{Borrowed, Owned, SourceType},
    ProgramContext,
};

/// General trait for global and local contexts / scopes
pub trait EvaluationContext<S: SourceType> {
    /// Look up a mapping, identified by it's name and arguments. First checks in the current
    /// scope, then its parent scope, then its parent scope and so on.
    fn lookup(&self, name: &PathIdent, args: &[Expr<Owned>]) -> Option<&Mapping<S>>;
}

/// Local scope (~= stack frame)
pub struct ScopedContext<'parent, S: SourceType> {
    /// Reference to the context below us (Global or another scope)
    pub parent: &'parent dyn EvaluationContext<S>,

    /// Local variables added by this scope
    pub locals: HashMap<String, Vec<Mapping<S>>>,
}

/// Merges another context into this one. Mutates `a` in place.
///
/// If a key (variable/function name) exists in both, the mappings
/// from `b` are appended to the list in `a`.
pub fn merge_contexts<S: SourceType>(a: &mut ProgramContext<S>, b: ProgramContext<S>) {
    for (key, mappings) in b {
        a.entry(key).or_default().extend(mappings);
    }
}

pub fn get_owned_context(ctx: ProgramContext<Borrowed<'_>>) -> ProgramContext<Owned> {
    ctx.into_iter()
        .map(|(key, mappings)| {
            let owned_key = key.to_owned();
            let owned_mappings = mappings
                .into_iter()
                .map(expandr_syntax::IntoOwned::into_owned)
                .collect();

            (owned_key, owned_mappings)
        })
        .collect()
}

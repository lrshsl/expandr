use std::collections::HashMap;

use crate::{
    ast::Mapping,
    source_type::{Borrowed, Owned, SourceType},
};

// General trait for Global and Local contexts / scopes
pub trait EvaluationContext<S: SourceType> {
    /// Looks up a mapping by name and immediately expands it.
    /// This hides whether the underlying mapping was Borrowed or Owned.
    fn lookup(&self, name: &str) -> Option<&Vec<Mapping<S>>>;
}

pub type ProgramContext<S> = HashMap<<S as SourceType>::Str, Vec<Mapping<S>>>;

impl<S: SourceType> EvaluationContext<S> for ProgramContext<S>
where
    S::Str: std::borrow::Borrow<str> + std::hash::Hash + Eq,
{
    fn lookup(&self, name: &str) -> Option<&Vec<Mapping<S>>> {
        self.get(name)
    }
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
                .map(crate::ast::IntoOwned::into_owned)
                .collect();

            (owned_key, owned_mappings)
        })
        .collect()
}

/// Local scope (~= stack frame)
pub struct ScopedContext<'parent, S>
where
    S: SourceType,
{
    /// Reference to the context below us (Global or another Scope)
    pub parent: &'parent dyn EvaluationContext<S>,

    /// Local variables added by this scope
    pub locals: HashMap<String, Vec<Mapping<S>>>,
}

impl<'parent, S: SourceType> EvaluationContext<S> for ScopedContext<'parent, S> {
    fn lookup(&self, name: &str) -> Option<&Vec<Mapping<S>>> {
        // Try lookup locally first
        if let Some(mappings) = self.locals.get(name) {
            return Some(mappings);
        }

        // Not found? Delegate to the parent
        self.parent.lookup(name)
    }
}

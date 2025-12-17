use std::collections::HashMap;

use crate::{
    ast::Mapping,
    errors::expansion_error::ExpansionResult,
    source_type::{Borrowed, Owned, SourceType},
};

pub type ProgramContext<S: SourceType> = HashMap<String, Vec<Mapping<S>>>;

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

pub trait Expandable<S: SourceType> {
    fn expand(self, ctx: &ProgramContext<S>) -> ExpansionResult;
}

#[derive(Clone, Debug)]
pub enum Expanded {
    Str(String),
    Int(i64),
}

impl PartialEq for Expanded {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Str(a), Self::Str(b)) => a == b,
            (Self::Int(a), Self::Int(b)) => a == b,
            (Self::Int(i), Self::Str(s)) | (Self::Str(s), Self::Int(i)) => {
                if !s.is_empty() || *i < 0 || *i > (u32::MAX as i64) {
                    false
                } else {
                    s.chars().next() == char::from_u32(*i as u32)
                }
            }
        }
    }
}

impl Expanded {
    pub fn into_string(self) -> String {
        match self {
            Self::Str(s) => s,
            Self::Int(i) => format!("{i}"),
        }
    }
}

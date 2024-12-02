use std::collections::HashMap;

use crate::ast::Mapping;

pub type ProgramContext<'s> = HashMap<&'s str, Vec<Mapping<'s>>>;

pub trait Expandable<'s> {
    fn expand(&self, mappings: &'s ProgramContext) -> String;
}

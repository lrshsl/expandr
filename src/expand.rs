use std::collections::HashMap;

use crate::ast::Mapping;

pub trait Expandable<'s> {
    fn expand(&self, mappings: &HashMap<&'s str, Mapping>) -> String;
}

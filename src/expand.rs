use std::collections::HashMap;

use crate::ast::Mapping;

pub type ProgramContext<'s> = HashMap<&'s str, Vec<Mapping<'s>>>;

pub trait Expandable<'s> {
    fn expand(self, ctx: &'s ProgramContext) -> Expanded;
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

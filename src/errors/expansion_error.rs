use std::fmt;

use crate::expand::Expanded;

pub type ExpansionResult = Result<Expanded, ExpansionError>;

#[derive(Debug, thiserror::Error)]
pub enum ExpansionError {
    UnknownMappingReferenced(String),
}

impl<'s> fmt::Display for ExpansionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExpansionError::UnknownMappingReferenced(s) => {
                write!(f, "[ExpansionError] Unknown mapping referenced: {s}")
            }
        }
    }
}

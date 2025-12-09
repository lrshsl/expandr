use std::io::Error as IoError;

use crate::{derive_from, errors::parse_error::ParseError};

pub type ExpansionResult<'s, T> = Result<T, ExpansionError<'s>>;

#[derive(Debug, thiserror::Error)]
pub enum ExpansionError<'s> {
    ParseError(ParseError<'s>),
    IoError(#[from] IoError),
}

derive_from!(ParseError for ExpansionError<'s>, lt<'s>);

impl std::fmt::Display for ExpansionError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ExpansionError::ParseError(error) => write!(f, "{}", error),
            ExpansionError::IoError(error) => write!(f, "{}", error),
        }
    }
}

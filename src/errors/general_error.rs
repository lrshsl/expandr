use std::io::Error as IoError;

use crate::{
    derive_from,
    errors::{expansion_error::ExpansionError, parse_error::ParseError},
};

pub type GeneralResult<'s, T> = Result<T, GeneralError<'s>>;

#[derive(Debug, thiserror::Error)]
pub enum GeneralError<'s> {
    ParseError(ParseError<'s>),
    IoError(#[from] IoError),
    ExpansionError(ExpansionError),
}

derive_from!(ParseError for GeneralError<'s>, lt<'s>);

impl std::fmt::Display for GeneralError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            GeneralError::ParseError(error) => write!(f, "{}", error),
            GeneralError::IoError(error) => write!(f, "{}", error),
            GeneralError::ExpansionError(error) => write!(f, "{}", error),
        }
    }
}

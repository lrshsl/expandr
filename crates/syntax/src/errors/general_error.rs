use std::io::Error as IoError;

use crate::{
    derive_from,
    errors::{expansion_error::ExpansionError, parse_error::ParseError},
};

pub type GeneralResult<T> = Result<T, GeneralError>;

#[derive(Debug, thiserror::Error)]
pub enum GeneralError {
    ParseError(ParseError),
    IoError(#[from] IoError),
    ExpansionError(ExpansionError),
}

derive_from!(ParseError for GeneralError);

impl std::fmt::Display for GeneralError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            GeneralError::ParseError(error) => write!(f, "{}", error),
            GeneralError::IoError(error) => write!(f, "{}", error),
            GeneralError::ExpansionError(error) => write!(f, "{}", error),
        }
    }
}

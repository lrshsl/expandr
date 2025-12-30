use std::{fmt, io::Error as IoError};

use expandr_semantic::{derive_from, expansion_error::ExpansionError};
use expandr_syntax::errors::parse_error::ParseError;

pub type GeneralResult<T> = Result<T, GeneralError>;

#[derive(Debug, thiserror::Error)]
pub enum GeneralError {
    ParseError(ParseError),
    IoError(#[from] IoError),
    ExpansionError(ExpansionError),
}

impl GeneralError {
    pub fn pretty_print(&self, f: &mut impl fmt::Write, color_codes: bool) -> fmt::Result {
        match self {
            GeneralError::ParseError(e) => e.pretty_print(f, color_codes),
            GeneralError::IoError(e) => write!(f, "{e}"),
            GeneralError::ExpansionError(e) => e.pretty_print(f, color_codes),
        }
    }
}

derive_from!(ParseError for GeneralError);

impl std::fmt::Display for GeneralError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        self.pretty_print(f, false)
    }
}

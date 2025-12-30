use std::fmt;

use crate::{
    errors::pretty_print::{print_err_ctx, print_raise_ctx},
    lexer::FileContext,
};

pub type LexerResult<T> = Result<T, LexerError>;

#[derive(Debug, thiserror::Error, PartialEq, Clone)]
pub enum LexerError {
    UnknownError {
        msg: String,
        ctx: Box<FileContext>,
        file: &'static str,
        line: u32,
    },
}

impl LexerError {
    pub fn ctx(&self) -> &FileContext {
        match self {
            Self::UnknownError { ctx, .. } => &ctx,
        }
    }
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.pretty_print(f, false)
    }
}

impl LexerError {
    fn pretty_print(&self, f: &mut fmt::Formatter<'_>, color_codes: bool) -> fmt::Result {
        match self {
            LexerError::UnknownError {
                msg,
                ctx,
                file,
                line,
            } => {
                // Print the location/context
                print_raise_ctx(f, file, *line, color_codes)?;
                print_err_ctx(f, ctx, color_codes)?;

                // Write the actual error
                writeln!(f, "|  Lexer error: {msg}")
            }
        }
    }
}

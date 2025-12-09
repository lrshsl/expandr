use std::fmt;

use crate::{
    errors::pretty_print::{print_err_ctx, print_raise_ctx},
    lexer::FileContext,
};

pub type LexerResult<'s, T> = Result<T, LexerError<'s>>;

#[derive(Debug, thiserror::Error, PartialEq, Clone)]
pub enum LexerError<'s> {
    UnknownError {
        msg: String,
        ctx: FileContext<'s>,
        file: &'static str,
        line: u32,
    },
}

impl<'s> fmt::Display for LexerError<'s> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexerError::UnknownError {
                msg,
                ctx,
                file,
                line,
            } => {
                // Print the location/context
                print_raise_ctx(file, *line);
                print_err_ctx(ctx);

                // Write the actual error
                write!(f, "|  Lexer error: {msg}\n")
            }
        }
    }
}

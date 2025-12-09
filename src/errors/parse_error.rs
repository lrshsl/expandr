use thiserror::Error;

use crate::{
    derive_from,
    errors::{
        lexer_error::LexerError,
        pretty_print::{print_err_ctx, print_raise_ctx},
    },
    lexer::FileContext,
};
use std::fmt;

pub type ParseResult<'s, T> = Result<T, ParseError<'s>>;

#[derive(Debug, Error)]
pub enum ParseError<'s> {
    LexerError(LexerError<'s>),
    UnexpectedToken {
        found: String,
        expected: Vec<String>,
        ctx: Box<FileContext<'s>>,
        file: &'static str,
        line: u32,
    },
    UnexpectedEof {
        ctx: Box<FileContext<'s>>,
        file: &'static str,
        line: u32,
    },
}

derive_from!(LexerError for ParseError<'s>, lt<'s>);

impl<'s> fmt::Display for ParseError<'s> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::UnexpectedToken {
                found,
                expected,
                ctx,
                file,
                line,
            } => {
                print_raise_ctx(file, *line);
                print_err_ctx(ctx);
                write!(
                    f,
                    "|  Unexpected token: \"{found}\"\n|  Expecting one of {expected:?}\n"
                )
            }
            ParseError::UnexpectedEof { ctx, file, line } => {
                print_raise_ctx(file, *line);
                print_err_ctx(ctx);
                writeln!(f, "|  Unexpected end of file")
            }
            ParseError::LexerError(err) => write!(f, "{err}"),
        }
    }
}

#[macro_export]
macro_rules! unexpected_token {
    (
        found    : $tok:expr,
        expected : [ $($expected:pat_param ),* $(,)? ],
        @ $ctx:expr
    ) => {
        {
            let found = format!("{:?}", $tok);
            let expected = vec![$( stringify!($expected).to_string() ),*];
            Err($crate::errors::parse_error::ParseError::UnexpectedToken {
                found,
                expected,
                ctx: $ctx,
                file: file!(),
                line: line!(),
            })
        }
    };
    (
        found    : $tok:expr,
        expected : $expected:expr,
        @ $ctx:expr
    ) => {
        {
            let found = format!("{:?}", $tok);
            let expected = vec![format!("{:?}", $expected)];
            Err($crate::errors::parse_error::ParseError::UnexpectedToken {
                found,
                expected,
                ctx: $ctx,
                file: file!(),
                line: line!(),
            })
        }
    };
}

#[macro_export]
macro_rules! unexpected_eof {
    ( $ctx:expr ) => {
        Err($crate::errors::parse_error::ParseError::UnexpectedEof {
            ctx: $ctx,
            file: file!(),
            line: line!(),
        })
    };
}

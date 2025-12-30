use crate::{
    derive_from,
    errors::{
        lexer_error::LexerError,
        pretty_print::{print_err_ctx, print_raise_ctx},
    },
    lexer::FileContext,
};
use std::fmt;

pub type ParseResult<'s, T> = Result<T, ParseError>;

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    LexerError(LexerError),
    UnexpectedToken {
        found: String,
        expected: Vec<String>,
        ctx: Box<FileContext>,
        file: &'static str,
        line: u32,
    },
    UnexpectedEof {
        ctx: Box<FileContext>,
        file: &'static str,
        line: u32,
    },
}

derive_from!(LexerError for ParseError);

impl ParseError {
    pub fn ctx(&self) -> &FileContext {
        match self {
            Self::LexerError(lexer_err) => lexer_err.ctx(),
            Self::UnexpectedToken { ctx, .. } => &ctx,
            Self::UnexpectedEof { ctx, .. } => &ctx,
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.pretty_print(f, false)
    }
}

impl ParseError {
    pub fn pretty_print(&self, f: &mut impl fmt::Write, color_codes: bool) -> fmt::Result {
        match self {
            ParseError::UnexpectedToken {
                found,
                expected,
                ctx,
                file,
                line,
            } => {
                print_raise_ctx(f, file, *line, color_codes)?;
                print_err_ctx(f, ctx, color_codes)?;
                write!(
                    f,
                    "|  Unexpected token: \"{found}\"\n|  Expecting one of {expected:?}\n"
                )
            }
            ParseError::UnexpectedEof { ctx, file, line } => {
                print_raise_ctx(f, file, *line, color_codes)?;
                print_err_ctx(f, ctx, color_codes)?;
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

use color_print::ceprintln;

use crate::lexer::{ExprToken, FileContext};

#[derive(Debug)]
pub enum ParsingError<'s> {
    AbruptEof(FileContext<'s>),
    UnexpectedToken(&'s str, FileContext<'s>, ExprToken<'s>, Vec<ExprToken<'s>>),
    TokenError(String),
}

#[macro_export]
macro_rules! unexpected_token {
    (
        found    : $tok:expr,
        expected : [ $($expected:pat_param ),* $(,)? ],
        @ $ctx:expr
) => {
        color_print::ceprintln!("> <red>Error</> occurred in <blue>{}:{}</>", file!(), line!());
        color_print::ceprintln!(
            "\t<bold>Unexpected token: {tok:?}\n\tExpected one of {exp:?}</bold>\n",
            tok = $tok,
            exp = [ $(stringify!($expected)),* ]
        );
        crate::errs::print_err_ctx($ctx);
        std::process::exit(1);
    };
}

#[macro_export]
macro_rules! unexpected_eof {
    ( $ctx:expr ) => {{
        color_print::ceprintln!("> <red>Error</> occurred in {}:{}\n", file!(), line!());
        color_print::ceprintln!("Unexpected end of file\n");
        crate::errs::print_err_ctx($ctx);
        std::process::exit(1);
    }};
}

pub fn print_err_ctx(file_ctx: &FileContext) {
    let FileContext {
        filename,
        line,
        column,
        cur_line,
        cur_slice,
        ..
    } = file_ctx;
    let token_len = cur_slice.len();
    let token_start = column - token_len;
    ceprintln!(
        r#"<blue>{filename}:{line}:{token_start}</> Error at '{cur_slice}'
{cur_line}
{padding}<red>{markers}</>
"#,
        padding = " ".repeat(token_start),
        markers = "^".repeat(token_len),
    );
}

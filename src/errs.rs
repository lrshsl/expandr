use color_print::ceprintln;

use crate::lexer::{ExprToken, FileContext};

#[macro_export]
macro_rules! log {
    ( $($e:expr),* ) => {{
        use std::io::Write;
        let mut f =
            std::fs::OpenOptions::new()
                .append(true)
                .create(true)
                .open("output/logs")
                .unwrap();
        writeln!(f, $($e),*).unwrap();
        f.flush().unwrap();
    }};
}

#[macro_export]
macro_rules! log_lexer {
    ( $($e:expr),* ) => {{
        use std::io::Write;
        let mut f =
            std::fs::OpenOptions::new()
                .append(true)
                .create(true)
                .open("output/tokens")
                .unwrap();
        writeln!(f, $($e),*).unwrap();
        f.flush().unwrap();
    }};
}

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
        content,
        line,
        cur_slice,
        ..
    } = file_ctx;
    let token_len = cur_slice.len();
    let token_start = file_ctx.token_start();

    let cur_line = content.lines().nth(*line - 1).expect("Line does not exist");

    ceprintln!(
        r#"<blue>{filename}:{line}:{token_start}</> Error at '{cur_slice}'
{cur_line}
{padding}<red>{markers}</>
"#,
        padding = " ".repeat(token_start - 1),
        markers = "^".repeat(token_len),
    );
}

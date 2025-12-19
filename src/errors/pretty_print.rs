use std::fmt;

use crate::lexer::FileContext;

pub fn print_raise_ctx(f: &mut fmt::Formatter, file: &str, line: u32) -> fmt::Result {
    color_print::cwrite!(
        f,
        "\n
| <bold,red>Error</> raised from <blue>{file}:{line}</>
|
"
    )
}

pub fn print_err_ctx(f: &mut fmt::Formatter, file_ctx: &FileContext) -> fmt::Result {
    let FileContext {
        filename,
        cur_line,
        line,
        cur_slice,
        ..
    } = file_ctx;
    let token_len = cur_slice.len();
    let token_start = file_ctx.token_start();

    color_print::cwrite!(
        f,
        "\
|  <blue>{filename}:{line}:{token_start}</> at '{cur_slice}'
|  <italic>{cur_line}</>
|  {padding}<red>{markers}</>
",
        padding = " ".repeat(token_start - 1),
        markers = "^".repeat(token_len),
    )
}

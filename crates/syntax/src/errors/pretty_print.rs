use std::fmt;

use crate::lexer::FileContext;

pub fn print_raise_ctx(f: &mut impl fmt::Write, file: &str, line: u32) -> fmt::Result {
    color_print::cwrite!(
        f,
        "\n\
| <bold,red>Error</> raised from <blue>{file}:{line}</>\n\
|\n"
    )
}

pub fn print_err_ctx(f: &mut impl fmt::Write, file_ctx: &FileContext) -> fmt::Result {
    let FileContext {
        source_name,
        cur_line,
        line,
        cur_slice,
        ..
    } = file_ctx;

    let token_len = cur_slice.len();
    let token_start = file_ctx.token_start();
    let filename = source_name.as_deref().unwrap_or("unknown");

    let padding = " ".repeat(token_start.saturating_sub(1));
    let markers = "^".repeat(token_len);

    color_print::cwrite!(
        f,
        "\
|  <blue>{filename}:{line}:{token_start}</> at '{cur_slice}'
|  <italic>{cur_line}</>
|  {padding}<red>{markers}</>
"
    )
}

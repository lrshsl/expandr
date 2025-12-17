use color_print::ceprint;

use crate::lexer::FileContext;

pub fn print_raise_ctx(file: &str, line: u32) {
    color_print::ceprint!(
        "\n
| <bold,red>Syntax error</> raised from <blue>{file}:{line}</>
|
"
    );
}

pub fn print_err_ctx(file_ctx: &FileContext) {
    let FileContext {
        filename,
        cur_line,
        line,
        cur_slice,
        ..
    } = file_ctx;
    let token_len = cur_slice.len();
    let token_start = file_ctx.token_start();

    ceprint!(
        "\
|  <blue>{filename}:{line}:{token_start}</> at '{cur_slice}'
|  <italic>{cur_line}</>
|  {padding}<red>{markers}</>
",
        padding = " ".repeat(token_start - 1),
        markers = "^".repeat(token_len),
    );
}

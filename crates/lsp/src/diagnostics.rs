use expandr_syntax::FileContext;
use tower_lsp::lsp_types::*;

use crate::server::ServerState;

fn get_range(ctx: &FileContext) -> tower_lsp::lsp_types::Range {
    let start_line = ctx.line as u32 - 1;
    let start_col = ctx.column as u32;

    tower_lsp::lsp_types::Range::new(
        Position::new(start_line, start_col),
        Position::new(
            start_line + ctx.cur_slice.chars().filter(|&ch| ch == '\n').count() as u32,
            start_col + ctx.cur_slice.len() as u32,
        ),
    )
}

impl ServerState {
    pub(crate) async fn publish_diagnostics(&self, uri: Url) {
        let files = self.files.read().await;
        let Some(src) = files.get(&uri) else {
            return;
        };

        let filename = uri
            .to_file_path()
            .unwrap()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        let diagnostics = match expandr_syntax::parse(src, Some(filename)) {
            Ok(_) => Vec::new(),
            Err(parse_err) => {
                vec![Diagnostic {
                    range: get_range(parse_err.ctx()),
                    severity: Some(DiagnosticSeverity::ERROR),
                    message: parse_err.to_string(),
                    ..Default::default()
                }]
            }
        };

        self.client
            .publish_diagnostics(uri, diagnostics, None)
            .await;
    }
}

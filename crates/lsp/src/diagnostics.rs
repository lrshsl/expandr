use tower_lsp::lsp_types::*;

use crate::server::ServerState;

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
                let ctx = parse_err.ctx();
                let range = tower_lsp::lsp_types::Range::new(
                    Position {
                        line: ctx.line as u32 - 1,
                        character: ctx.column as u32,
                    },
                    Position {
                        line: ctx.line as u32 - 1,
                        character: (ctx.column + ctx.cur_slice.len()) as u32,
                    },
                );
                vec![Diagnostic {
                    range,
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

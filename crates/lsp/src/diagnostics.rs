use tower_lsp::lsp_types::*;

use crate::server::ServerState;

impl ServerState {
    pub(crate) async fn publish_diagnostics(&self, uri: Url) {
        let files = self.files.read().await;
        let src = match files.get(&uri) {
            Some(s) => s,
            None => return,
        };

        let diagnostics = match expandr_syntax::parse(src) {
            Ok(_) => Vec::new(),
            Err(e) => vec![Diagnostic {
                range: Range::default(),
                severity: Some(DiagnosticSeverity::ERROR),
                message: e.to_string(),
                ..Default::default()
            }],
        };

        self.client
            .publish_diagnostics(uri, diagnostics, None)
            .await;
    }
}

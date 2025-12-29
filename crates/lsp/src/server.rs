use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};

use std::collections::HashMap;
use tokio::sync::RwLock;

pub struct ServerState {
    pub(crate) client: Client,
    pub(crate) files: RwLock<HashMap<Url, String>>,
}

impl ServerState {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            files: RwLock::new(HashMap::new()),
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for ServerState {
    async fn initialize(
        &self,
        _: InitializeParams,
    ) -> tower_lsp::jsonrpc::Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                ..Default::default()
            },
            server_info: Some(ServerInfo {
                name: "expandr-lsp".into(),
                version: Some("0.1.0".into()),
            }),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "LSP initialized")
            .await;
    }

    async fn shutdown(&self) -> tower_lsp::jsonrpc::Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri.clone();
        let text = params.text_document.text;

        self.files.write().await.insert(uri.clone(), text);

        self.publish_diagnostics(uri).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri.clone();
        let text = params.content_changes[0].text.clone();

        self.files.write().await.insert(uri.clone(), text);

        self.publish_diagnostics(uri).await;
    }
}

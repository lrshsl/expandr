use crate::server::ServerState;
use tower_lsp::{LspService, Server};

mod diagnostics;
mod server;

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| ServerState::new(client));
    Server::new(stdin, stdout, socket).serve(service).await;
}

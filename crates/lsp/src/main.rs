use tower_lsp::{LspService, Server};
use crate::server::ServerState;

mod server;
mod diagnostics;

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| ServerState::new(client));
    Server::new(stdin, stdout, socket).serve(service).await;
}

use std::sync::Mutex;

use bis_lsp::ParseAnalyzer;
use bis_lsp::hover::{HoverContext, HoverEngine};
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};
use tree_sitter::Parser;

use std::collections::HashMap;
use tower_lsp::lsp_types::Url;

struct DocumentState {
    text: String,
    tree: Option<tree_sitter::Tree>,
}

struct Backend {
    client: Client,
    parser: Mutex<Parser>,
    documents: Mutex<HashMap<Url, DocumentState>>,
    hover_engine: HoverEngine,
}

impl Backend {
    fn new(client: Client) -> Self {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_bis::LANGUAGE.into())
            .expect("Failed to load grammar.");

        Self {
            client,
            parser: Mutex::new(parser),
            documents: Mutex::new(HashMap::new()),
            hover_engine: HoverEngine::new(),
        }
    }
}

impl Backend {
    async fn on_document_change(&self, uri: Url, text: String) {
        let tree = self.parser.lock().unwrap().parse(&text, None);

        let parse_diagnostics = if let Some(ref tree) = tree {
            match ParseAnalyzer::analyze(&tree.root_node()) {
                Ok(d) => d,
                Err(e) => {
                    self.client
                        .log_message(MessageType::ERROR, format!("Parse error: {:?}", e))
                        .await;
                    Vec::new()
                }
            }
        } else {
            Vec::new()
        };

        // Store the document state for hover/other features
        self.documents
            .lock()
            .unwrap()
            .insert(uri.clone(), DocumentState { text, tree });

        self.client
            .publish_diagnostics(uri, parse_diagnostics, None)
            .await;
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "BIS Server initialized.")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        self.client
            .log_message(MessageType::INFO, "BIS Server shutting down.")
            .await;

        Ok(())
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = params.text_document_position_params.text_document.uri;
        let pos = params.text_document_position_params.position;

        let docs = self.documents.lock().unwrap();
        let Some(doc) = docs.get(&uri) else {
            return Ok(None);
        };
        let Some(ref tree) = doc.tree else {
            return Ok(None);
        };

        let ctx = HoverContext::new(tree, &doc.text, pos)?;
        Ok(self.hover_engine.hover(&ctx))
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let text = params.text_document.text;
        self.on_document_change(uri, text).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        if let Some(change) = params.content_changes.into_iter().next() {
            self.on_document_change(uri, change.text).await;
        }
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();
    let (service, socket) = LspService::new(Backend::new);
    Server::new(stdin, stdout, socket).serve(service).await;
}

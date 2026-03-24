use anyhow::Result;
use tower_lsp::lsp_types::Diagnostic;
use tree_sitter::{Node, Parser, TreeCursor};

pub struct ParseAnalyzer;

impl ParseAnalyzer {
    pub fn analyze(node: &Node) -> Result<Vec<Diagnostic>> {
        let diagnostics = Vec::new();

        let mut cursor = node.walk();

        Ok(diagnostics)
    }
}

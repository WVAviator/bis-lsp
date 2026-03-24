use tower_lsp::lsp_types::{Hover, HoverContents, MarkupContent, MarkupKind, Position, Range};

use crate::hover::{HoverContext, HoverProvider, node_to_lsp_range};

pub struct VariableHoverProvider;

impl VariableHoverProvider {
    pub fn new() -> Self {
        Self {}
    }
}

impl HoverProvider for VariableHoverProvider {
    fn provide(&self, ctx: &HoverContext) -> Option<Hover> {
        let node = ctx.node;

        match node.kind() {
            "named_variable" | "env_variable" | "global_variable" | "numbered_variable" => {}
            _ => return None,
        }

        // Grab the variable text from the source
        let var_text = &ctx.source[node.byte_range()];

        let markdown = format!("**Variable**: `{}`\n\nType: `{}`", var_text, node.kind());

        Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: markdown,
            }),
            range: Some(node_to_lsp_range(node)),
        })
    }
}

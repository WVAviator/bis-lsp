use std::collections::HashMap;

use tower_lsp::lsp_types::{Hover, HoverContents, MarkupContent, MarkupKind};

use crate::hover::{HoverContext, HoverProvider, node_to_lsp_range};

pub struct CommandHoverProvider {
    docs: HashMap<&'static str, &'static str>,
}

impl CommandHoverProvider {
    pub fn new() -> Self {
        let mut docs = HashMap::new();
        docs.insert("LDV", include_str!("../../docs/commands/LDV.md"));
        // docs.insert("LCV", include_str!("../../docs/commands/LCV.md"));
        // docs.insert("RNM", include_str!("../../docs/commands/RNM.md"));
        // docs.insert("SRH", include_str!("../../docs/commands/SRH.md"));
        Self { docs }
    }
}

impl HoverProvider for CommandHoverProvider {
    fn provide(&self, ctx: &HoverContext) -> Option<Hover> {
        let node = ctx.node;

        if node.kind() != "call" {
            return None;
        }

        let command = ctx.source[node.byte_range()].to_uppercase();
        let markdown = self.docs.get(command.as_str())?;

        Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: markdown.to_string(),
            }),
            range: Some(node_to_lsp_range(node)),
        })
    }
}

use std::collections::HashMap;

use tower_lsp::lsp_types::{Hover, HoverContents, MarkupContent, MarkupKind};

use crate::hover::{HoverContext, HoverProvider, node_to_lsp_range};

pub struct OptionsHoverProvider {
    docs: HashMap<&'static str, HashMap<String, String>>,
}

#[derive(serde::Deserialize)]
struct CommandOptions {
    options: HashMap<String, String>,
}

impl OptionsHoverProvider {
    pub fn new() -> Self {
        let mut docs = HashMap::new();

        let ldv: CommandOptions = toml::from_str(include_str!("../../docs/options/LDV.toml"))
            .expect("Failed to parse LDV.toml.");
        let srh: CommandOptions = toml::from_str(include_str!("../../docs/options/SRH.toml"))
            .expect("Failed to parse SRH.toml.");
        // SRH has the same options as SRU
        let sru: CommandOptions = toml::from_str(include_str!("../../docs/options/SRH.toml"))
            .expect("Failed to parse SRH.toml.");

        docs.insert("LDV", ldv.options);
        docs.insert("SRH", srh.options);
        docs.insert("SRU", sru.options);
        // docs.insert("LCV", include_str!("../../docs/commands/LCV.md"));
        // docs.insert("RNM", include_str!("../../docs/commands/RNM.md"));
        // docs.insert("SRH", include_str!("../../docs/commands/SRH.md"));
        Self { docs }
    }
}

impl HoverProvider for OptionsHoverProvider {
    fn provide(&self, ctx: &HoverContext) -> Option<Hover> {
        let mut node = ctx.node;

        while node.kind() != "option" {
            node = node.parent()?;
        }

        let parent = node.parent()?;
        let command = parent.kind().to_uppercase();

        let option_docs = self.docs.get(command.as_str())?;

        let mut cursor = parent.walk();
        let used_options: Vec<_> = parent
            .children(&mut cursor)
            .filter(|child| child.kind() == "option")
            .filter_map(|child| {
                let opt = ctx.source[child.byte_range()].to_uppercase();

                let doc = option_docs.get(opt.get(0..1)?)?;
                let highlight = if child.id() == node.id() {
                    "→ "
                } else {
                    "  "
                };
                Some(format!("{highlight} {doc}"))
            })
            .collect();

        if used_options.is_empty() {
            return None;
        }

        let markdown = format!("**@{}** Options:\n\n{}", command, used_options.join("\n"));

        Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: markdown,
            }),
            range: Some(node_to_lsp_range(node)),
        })
    }
}

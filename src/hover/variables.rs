use std::collections::HashMap;

use tower_lsp::lsp_types::{Hover, HoverContents, MarkupContent, MarkupKind, Position, Range};
use tree_sitter::StreamingIterator;

use crate::hover::{HoverContext, HoverProvider, node_to_lsp_range};

pub struct VariableHoverProvider {
    type_docs: TypeDocs,
    reserved_word_docs: ReservedWordDocs,
}

#[derive(serde::Deserialize)]
struct TypeDoc {
    title: String,
    doc: String,
}

#[derive(serde::Deserialize)]
struct TypeDocs {
    types: HashMap<String, TypeDoc>,
}

#[derive(serde::Deserialize)]
struct ReservedWordDoc {
    #[serde(rename = "type")]
    type_str: String,
    description: String,
}

#[derive(serde::Deserialize)]
struct ReservedWordDocs {
    reserved_words: HashMap<String, ReservedWordDoc>,
}

impl VariableHoverProvider {
    pub fn new() -> Self {
        let type_docs: TypeDocs = toml::from_str(include_str!("../../docs/variables/types.toml"))
            .expect("Could not parse types.toml.");
        let reserved_word_docs =
            toml::from_str(include_str!("../../docs/variables/reserved_words.toml"))
                .expect("Could not parse reserved_words.toml");
        Self {
            type_docs,
            reserved_word_docs,
        }
    }
}

impl HoverProvider for VariableHoverProvider {
    fn provide(&self, ctx: &HoverContext) -> Option<Hover> {
        let node = ctx.node;

        match node.kind() {
            "named_variable" | "env_variable" | "global_variable" | "numbered_variable" => {
                let var_text = &ctx.source[node.byte_range()];
                let var_type = find_variable_type(&ctx, var_text, node.kind(), node)
                    .unwrap_or("?".to_string());

                let markdown = match var_type.as_str() {
                    "?" => format!("**Variable**: `{}`\nType: `{}`", var_text, var_type),
                    t => {
                        let (type_char, type_size) = t.split_at(1);
                        if let Some(type_doc) =
                            self.type_docs.types.get(type_char.to_uppercase().as_str())
                        {
                            format!(
                                "**Variable**: `{}{}`\nType: `{}`\nSize: {}",
                                var_text, var_type, type_doc.title, type_size
                            )
                        } else {
                            format!("**Variable**: `{}`\nType: `{}`", var_text, var_type)
                        }
                    }
                };

                Some(Hover {
                    contents: HoverContents::Markup(MarkupContent {
                        kind: MarkupKind::Markdown,
                        value: markdown,
                    }),
                    range: Some(node_to_lsp_range(node)),
                })
            }
            "type" => {
                let type_text = &ctx.source[node.byte_range()];
                let (type_char, type_size) = type_text.split_at(1);
                if let Some(type_doc) = self.type_docs.types.get(type_char.to_uppercase().as_str())
                {
                    let markdown = format!(
                        "Type: {} ({})\nSize: {}\n\n{}",
                        type_char, type_doc.title, type_size, type_doc.doc
                    );

                    return Some(Hover {
                        contents: HoverContents::Markup(MarkupContent {
                            kind: MarkupKind::Markdown,
                            value: markdown,
                        }),
                        range: Some(node_to_lsp_range(node)),
                    });
                }

                None
            }
            "reserved_word" => {
                let rw_text = &ctx.source[node.byte_range()];
                if let Some(rw_doc) = self
                    .reserved_word_docs
                    .reserved_words
                    .get(rw_text.to_uppercase().as_str())
                {
                    let type_string = match rw_doc.type_str.as_str() {
                        "" => String::from(""),
                        t => format!("Type: {}\n", t),
                    };

                    let markdown = format!(
                        "Reserved Word: {}\n{}\n{}",
                        rw_text, type_string, rw_doc.description
                    );

                    return Some(Hover {
                        contents: HoverContents::Markup(MarkupContent {
                            kind: MarkupKind::Markdown,
                            value: markdown,
                        }),
                        range: Some(node_to_lsp_range(node)),
                    });
                }

                None
            }
            _ => None,
        }
    }
}

fn find_variable_type(
    ctx: &HoverContext,
    var_name: &str,
    node_kind: &str,
    from: tree_sitter::Node,
) -> Option<String> {
    let query_str = format!(
        r#"
        (
            ({}) @var
            (type) @type
        )
        "#,
        node_kind
    );

    let query = tree_sitter::Query::new(&tree_sitter_bis::LANGUAGE.into(), &query_str).ok()?;

    let var_idx = query.capture_index_for_name("var")?;
    let type_idx = query.capture_index_for_name("type")?;

    let mut qcursor = tree_sitter::QueryCursor::new();
    qcursor.set_byte_range(0..from.start_byte());

    let mut matches = qcursor.matches(&query, ctx.tree.root_node(), ctx.source.as_bytes());

    let mut result = None;
    while let Some(m) = matches.next() {
        let var_node = m.captures.iter().find(|c| c.index == var_idx);
        let type_node = m.captures.iter().find(|c| c.index == type_idx);

        if let (Some(var_cap), Some(type_cap)) = (var_node, type_node) {
            if &ctx.source[var_cap.node.byte_range()] == var_name {
                result = Some(ctx.source[type_cap.node.byte_range()].to_string());
            }
        }
    }

    result
}

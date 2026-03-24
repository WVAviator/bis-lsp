use tower_lsp::{
    jsonrpc::{Error, Result},
    lsp_types::{Hover, Position, Range},
};

use crate::hover::{
    commands::CommandHoverProvider, options::OptionsHoverProvider, variables::VariableHoverProvider,
};

mod commands;
mod options;
mod variables;

pub struct HoverEngine {
    providers: Vec<Box<dyn HoverProvider>>,
}

impl HoverEngine {
    pub fn new() -> Self {
        Self {
            providers: vec![
                Box::new(CommandHoverProvider::new()),
                Box::new(VariableHoverProvider::new()),
                Box::new(OptionsHoverProvider::new()),
            ],
        }
    }

    pub fn hover(&self, ctx: &HoverContext) -> Option<Hover> {
        self.providers.iter().find_map(|p| p.provide(ctx))
    }
}

pub trait HoverProvider: Send + Sync {
    fn provide(&self, ctx: &HoverContext) -> Option<Hover>;
}

pub struct HoverContext<'a> {
    pub tree: &'a tree_sitter::Tree,
    pub source: &'a str,
    pub node: tree_sitter::Node<'a>,
}

impl<'a> HoverContext<'a> {
    pub fn new(tree: &'a tree_sitter::Tree, source: &'a str, position: Position) -> Result<Self> {
        let point = tree_sitter::Point {
            row: position.line as usize,
            column: position.character as usize,
        };

        let node = tree
            .root_node()
            .descendant_for_point_range(point, point)
            .ok_or(Error::new(tower_lsp::jsonrpc::ErrorCode::ParseError))?;
        Ok(Self { tree, source, node })
    }
}

pub fn node_to_lsp_range(node: tree_sitter::Node) -> Range {
    let start = node.start_position();
    let end = node.end_position();
    Range {
        start: Position::new(start.row as u32, start.column as u32),
        end: Position::new(end.row as u32, end.column as u32),
    }
}

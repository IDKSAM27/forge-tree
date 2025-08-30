pub mod tree_parser;

pub use tree_parser::TreeParser;

use crate::{Result, ForgeTreeError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectStructure {
    pub root: String,
    pub items: Vec<StructureItem>,
    pub variables: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructureItem {
    pub name: String,
    pub path: String,
    pub item_type: ItemType,
    pub template: Option<String>,
    pub content: Option<String>,
    pub children: Vec<StructureItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ItemType {
    Directory,
    File,
}

pub struct Parser {
    tree_parser: TreeParser,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            tree_parser: TreeParser::new(),
        }
    }

    pub fn parse(&self, input: &str) -> Result<ProjectStructure> {
        self.tree_parser.parse(input)
    }

    pub fn parse_file(&self, path: &str) -> Result<ProjectStructure> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| ForgeTreeError::Io(e))?;
        self.parse(&content)
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}

use crate::parser::{ItemType, ProjectStructure, StructureItem};
use crate::{Result, ForgeTreeError};
use regex::Regex;
use std::collections::HashMap;

pub struct TreeParser {
    tree_chars: Regex,
}

impl TreeParser {
    pub fn new() -> Self {
        Self {
            tree_chars: Regex::new(r"^([│├└─\s]*)(.*?)/?$").unwrap(),
        }
    }

    pub fn parse(&self, input: &str) -> Result<ProjectStructure> {
        let lines: Vec<&str> = input.lines().collect();
        if lines.is_empty() {
            return Err(ForgeTreeError::Parse("Empty input".to_string()));
        }

        let root_name = self.extract_root_name(&lines)?;
        
        // Parse all lines after the root
        let child_lines: Vec<&str> = lines[1..].iter()
            .filter(|line| !line.trim().is_empty())
            .copied()
            .collect();
        
        let items = self.parse_structure(&child_lines)?;

        Ok(ProjectStructure {
            root: root_name,
            items,
            variables: HashMap::new(),
        })
    }

    fn extract_root_name(&self, lines: &[&str]) -> Result<String> {
        let first_line = lines.first()
            .ok_or_else(|| ForgeTreeError::Parse("No root directory found".to_string()))?;
        
        let name = first_line.trim().trim_end_matches('/');
        if name.is_empty() {
            return Err(ForgeTreeError::Parse("Invalid root directory name".to_string()));
        }
        
        Ok(name.to_string())
    }

    fn parse_structure(&self, lines: &[&str]) -> Result<Vec<StructureItem>> {
        let mut items = Vec::new();
        let mut i = 0;

        while i < lines.len() {
            let line = lines[i];
            let current_depth = self.get_depth(line);
            
            let (name, is_directory) = self.parse_line(line)?;
            let mut item = StructureItem {
                name: name.clone(),
                path: name,
                item_type: if is_directory { ItemType::Directory } else { ItemType::File },
                template: None,
                content: None,
                children: Vec::new(),
            };

            // Collect children (lines with greater depth)
            i += 1;
            let mut child_lines = Vec::new();
            
            while i < lines.len() {
                let child_line = lines[i];
                let child_depth = self.get_depth(child_line);
                
                // If we hit a line at same or less depth, stop collecting children
                if child_depth <= current_depth {
                    break;
                }
                
                child_lines.push(child_line);
                i += 1;
            }

            // Recursively parse children
            if !child_lines.is_empty() {
                item.children = self.parse_structure(&child_lines)?;
                item.item_type = ItemType::Directory; // Has children, must be directory
            }

            items.push(item);
        }

        Ok(items)
    }

    fn get_depth(&self, line: &str) -> usize {
        let mut depth = 0;
        
        for ch in line.chars() {
            match ch {
                '├' | '└' | '│' => depth += 1,
                '─' | ' ' => continue, // Skip dashes and spaces
                _ => break, // Stop at first non-tree character
            }
        }
        
        depth
    }

    fn parse_line(&self, line: &str) -> Result<(String, bool)> {
        // Skip tree characters by counting characters, not bytes
        let content = line.chars()
            .skip_while(|&ch| ch == '│' || ch == '├' || ch == '└' || ch == '─' || ch == ' ')
            .collect::<String>()
            .trim()
            .to_string();
        
        if content.is_empty() {
            return Err(ForgeTreeError::Parse(format!("Empty name in line: {}", line)));
        }

        // Determine if it's a directory or file
        let is_directory = content.ends_with('/') || !content.contains('.');
        let clean_name = content.trim_end_matches('/').to_string();
        
        Ok((clean_name, is_directory))
    }
}

impl Default for TreeParser {
    fn default() -> Self {
        Self::new()
    }
}

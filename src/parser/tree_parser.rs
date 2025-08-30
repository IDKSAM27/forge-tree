use crate::parser::{ItemType, ProjectStructure, StructureItem};
use crate::{Result, ForgeTreeError};
use regex::Regex;
use std::collections::HashMap;

pub struct TreeParser {
    tree_chars: Regex,
    indent_detector: Regex,
}

impl TreeParser {
    pub fn new() -> Self {
        Self {
            tree_chars: Regex::new(r"^([│├└─\s]*)(.*?)/?$").unwrap(),
            indent_detector: Regex::new(r"^(\s*)").unwrap(),
        }
    }

    pub fn parse(&self, input: &str) -> Result<ProjectStructure> {
        let lines: Vec<&str> = input.lines().collect();
        if lines.is_empty() {
            return Err(ForgeTreeError::Parse("Empty input".to_string()));
        }

        let root_name = self.extract_root_name(&lines)?;
        let items = self.parse_lines(&lines[1..], 0)?;

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

    fn parse_lines(&self, lines: &[&str], base_indent: usize) -> Result<Vec<StructureItem>> {
        let mut items = Vec::new();
        let mut i = 0;

        while i < lines.len() {
            let line = lines[i];
            if line.trim().is_empty() {
                i += 1;
                continue;
            }

            let indent = self.get_indent_level(line);
            if indent < base_indent {
                break;
            }
            if indent > base_indent {
                i += 1;
                continue;
            }

            let (name, is_directory) = self.parse_line(line)?;
            let mut item = StructureItem {
                name: name.clone(),
                path: name,
                item_type: if is_directory { ItemType::Directory } else { ItemType::File },
                template: None,
                content: None,
                children: Vec::new(),
            };

            // Look ahead for children
            i += 1;
            let mut child_lines = Vec::new();
            while i < lines.len() {
                let child_line = lines[i];
                if child_line.trim().is_empty() {
                    i += 1;
                    continue;
                }
                
                let child_indent = self.get_indent_level(child_line);
                if child_indent <= indent {
                    break;
                }
                
                child_lines.push(child_line);
                i += 1;
            }

            if !child_lines.is_empty() {
                item.children = self.parse_lines(&child_lines, indent + 1)?;
                item.item_type = ItemType::Directory; // Has children, must be directory
            }

            items.push(item);
        }

        Ok(items)
    }

    fn parse_line(&self, line: &str) -> Result<(String, bool)> {
        if let Some(captures) = self.tree_chars.captures(line) {
            let name = captures.get(2)
                .ok_or_else(|| ForgeTreeError::Parse(format!("Invalid line format: {}", line)))?
                .as_str()
                .trim();

            if name.is_empty() {
                return Err(ForgeTreeError::Parse(format!("Empty name in line: {}", line)));
            }

            let is_directory = name.ends_with('/') || 
                              !name.contains('.') ||
                              line.contains("├──") ||
                              line.contains("└──");

            let clean_name = name.trim_end_matches('/').to_string();
            
            Ok((clean_name, is_directory))
        } else {
            Err(ForgeTreeError::Parse(format!("Failed to parse line: {}", line)))
        }
    }

    
    fn get_indent_level(&self, line: &str) -> usize {
        if line.trim().is_empty() {
            return 0;
        }
        
        let mut level = 0;
        let mut chars = line.chars().peekable();
        
        // Count indentation more intelligently
        while let Some(&ch) = chars.peek() {
            match ch {
                // Tree drawing characters each represent one level
                '├' | '└' => {
                    chars.next();
                    // Skip following dashes and spaces
                    while let Some(&next_ch) = chars.peek() {
                        if next_ch == '─' || next_ch == ' ' {
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    level += 1;
                }
                '│' => {
                    chars.next();
                    // Skip following space
                    if let Some(' ') = chars.peek() {
                        chars.next();
                    }
                    level += 1;
                }
                ' ' => {
                    // Count spaces in groups (typically 2-4 spaces per level)
                    let mut space_count = 0;
                    while let Some(' ') = chars.peek() {
                        chars.next();
                        space_count += 1;
                    }
                    // Each group of 2+ spaces represents roughly one level
                    if space_count > 0 {
                        level += (space_count + 1) / 2;
                    }
                }
                _ => break,
            }
        }
        
        level
    }

}

impl Default for TreeParser {
    fn default() -> Self {
        Self::new()
    }
}


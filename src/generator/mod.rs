pub mod file_generator;
pub mod template_engine;

pub use file_generator::FileGenerator;
pub use template_engine::TemplateEngine;

use crate::parser::ProjectStructure;
use crate::Result;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::Path;

pub struct Generator {
    file_generator: FileGenerator,
    template_engine: TemplateEngine,
    verbose: bool,
}

impl Generator {
    pub fn new() -> Self {
        Self {
            file_generator: FileGenerator::new(),
            template_engine: TemplateEngine::new(),
            verbose: false,
        }
    }

    pub fn with_verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }

    pub fn generate<P: AsRef<Path>>(&self, structure: &ProjectStructure, output_path: P) -> Result<()> {
        let output_path = output_path.as_ref();
        
        // Create progress bar
        let total_items = self.count_items(&structure.items);
        let pb = ProgressBar::new(total_items as u64);
        pb.set_style(
            ProgressStyle::with_template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}"
            ).unwrap()
        );

        // Create root directory
        let root_path = output_path.join(&structure.root);
        self.file_generator.create_directory(&root_path)?;
        
        if self.verbose {
            println!("{} {}", "Created".green().bold(), root_path.display());
        }

        // Generate all items
        self.generate_items(&structure.items, &root_path, &pb)?;
        
        pb.finish_with_message("Generation complete!");
        
        println!("\n{} Project '{}' generated successfully at {}", 
                 "✅".green(), 
                 structure.root.cyan().bold(), 
                 output_path.display());

        Ok(())
    }

    fn generate_items<P: AsRef<Path>>(
        &self, 
        items: &[crate::parser::StructureItem], 
        base_path: P,
        pb: &ProgressBar
    ) -> Result<()> {
        let base_path = base_path.as_ref();

        for item in items {
            let item_path = base_path.join(&item.name);
            
            match item.item_type {
                crate::parser::ItemType::Directory => {
                    self.file_generator.create_directory(&item_path)?;
                    if self.verbose {
                        println!("{} {}", "Created".green().bold(), item_path.display());
                    }
                    
                    // Recursively generate children
                    self.generate_items(&item.children, &item_path, pb)?;
                }
                crate::parser::ItemType::File => {
                    let content = if let Some(template) = &item.template {
                        self.template_engine.render_template(template, &std::collections::HashMap::new())?
                    } else {
                        item.content.clone().unwrap_or_default()
                    };
                    
                    self.file_generator.create_file(&item_path, &content)?;
                    if self.verbose {
                        println!("{} {}", "Created".blue().bold(), item_path.display());
                    }
                }
            }
            
            pb.inc(1);
            pb.set_message(format!("Processing {}", item.name));
        }

        Ok(())
    }

    fn count_items(&self, items: &[crate::parser::StructureItem]) -> usize {
        let mut count = items.len();
        for item in items {
            count += self.count_items(&item.children);
        }
        count
    }
}

impl Default for Generator {
    fn default() -> Self {
        Self::new()
    }
}

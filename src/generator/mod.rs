//! Project generation module

pub mod file_generator;
pub mod template_engine;

pub use file_generator::FileGenerator;
pub use template_engine::TemplateEngine;

use crate::parser::ProjectStructure;
use crate::Result;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::Path;

/// Main generator struct that coordinates project creation
pub struct Generator {
    template_engine: TemplateEngine,
    verbose: bool,
    force_override: bool,
}

impl Generator {
    pub fn new() -> Self {
        Self {
            template_engine: TemplateEngine::new(),
            verbose: false,
            force_override: false,
        }
    }

    pub fn with_verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }

    pub fn with_force_override(mut self, force: bool) -> Self {
        self.force_override = force;
        self
    }

    pub fn generate<P: AsRef<Path>>(&self, structure: &ProjectStructure, output_path: P) -> Result<()> {
        let output_path = output_path.as_ref();
        let total_items = self.count_items(&structure.items);
        
        // Create the root project directory
        let root_path = output_path.join(&structure.root);
        
        // Create FileGenerator with force override setting
        let file_generator = FileGenerator::new().with_force_overwrite(self.force_override);
        file_generator.create_directory(&root_path)?;
        
        // Set up progress bar
        let pb = ProgressBar::new(total_items as u64);
        pb.set_style(
            ProgressStyle::with_template(
                "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} {msg}"
            ).unwrap()
            .progress_chars("█▉▊▋▌▍▎▏  ")
        );

        if self.verbose {
            pb.println(format!("{} {}", "Created".green().bold(), root_path.display()));
        }

        // Generate all items
        self.generate_items(&structure.items, &root_path, &pb, &file_generator)?;
        
        pb.finish_with_message("Generation complete!");
        
        println!("\n{} Project '{}' forged successfully at {}", 
                 "✅".green(), 
                 structure.root.cyan().bold(), 
                 root_path.canonicalize()
                     .unwrap_or_else(|_| root_path.to_path_buf())
                     .display());

        Ok(())
    }

    fn generate_items<P: AsRef<Path>>(
        &self, 
        items: &[crate::parser::StructureItem], 
        base_path: P,
        pb: &ProgressBar,
        file_generator: &FileGenerator
    ) -> Result<()> {
        let base_path = base_path.as_ref();

        for item in items {
            let item_path = base_path.join(&item.name);
            
            pb.inc(1);
            pb.set_message(format!("Processing {}", item.name));
            
            match item.item_type {
                crate::parser::ItemType::Directory => {
                    file_generator.create_directory(&item_path)?;
                    if self.verbose {
                        pb.println(format!("{} {}", "Created".green().bold(), item_path.display()));
                    }
                    
                    self.generate_items(&item.children, &item_path, pb, file_generator)?;
                }
                crate::parser::ItemType::File => {
                    let content = if let Some(template) = &item.template {
                        self.template_engine.render_template(template, &std::collections::HashMap::new())?
                    } else {
                        item.content.clone().unwrap_or_default()
                    };
                    
                    file_generator.create_file(&item_path, &content)?;
                    if self.verbose {
                        pb.println(format!("{} {}", "Created".blue().bold(), item_path.display()));
                    }
                }
            }
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

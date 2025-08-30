//! Basic usage example for forge-tree
//! 
//! This example demonstrates how to use forge-tree programmatically

use forge_tree::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Forge-Tree Basic Usage Example");
    
    // Example structure as a string
    let structure_text = r#"
example-project/
├── src/
│   ├── main.rs
│   └── lib.rs
├── tests/
│   └── integration_test.rs
├── Cargo.toml
└── README.md
"#;

    println!("📝 Parsing structure...");
    
    // Parse the structure
    let parser = Parser::new();
    let structure = parser.parse(structure_text)?;
    
    println!("🌳 Root: {}", structure.root);
    println!("📁 Items: {}", count_items(&structure.items));
    
    // Generate the project (commented out to avoid creating files in example)
    // let generator = Generator::new().with_verbose(true);
    // generator.generate(&structure, "./example-output")?;
    
    println!("✅ Example completed successfully!");
    println!("💡 To actually generate files, use the CLI: forge-tree forge structure.txt");
    
    Ok(())
}

// Helper function to count items
fn count_items(items: &[forge_tree::parser::StructureItem]) -> usize {
    let mut count = items.len();
    for item in items {
        count += count_items(&item.children);
    }
    count
}

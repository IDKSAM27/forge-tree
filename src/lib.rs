//! # Forge-Tree 
//! 
//! A powerful project scaffolding tool that generates folder and file structures
//! from text representations.
//! 
//! ## Quick Start
//! 
//! ```
//! use forge_tree::{Parser};
//! 
//! let structure = r#"my-awesome-project/
//! ├── src/
//! │   ├── main.rs
//! │   └── lib.rs
//! └── Cargo.toml"#;
//! 
//! let parsed = Parser::new().parse(structure).unwrap();
//! assert_eq!(parsed.root, "my-awesome-project");
//! assert_eq!(parsed.items.len(), 2); // src/ and Cargo.toml
//! println!("Parsed project: {}", parsed.root);
//! ```

pub mod cli;
pub mod config;
pub mod error;
pub mod generator;
pub mod parser;

pub use error::{Result, ForgeTreeError};
pub use generator::Generator;
pub use parser::Parser;

/// Re-export commonly used types
pub mod prelude {
    pub use crate::{Generator, Parser, Result, ForgeTreeError};
}

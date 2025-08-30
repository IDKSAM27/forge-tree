//! # Forge-Tree
//! 
//! A powerful project scaffolding tool that generates folder and file structures
//! from text representations. Forge your project structure from simple text!
//! 
//! ## Quick Start
//! 
//! ```
//! use forge_tree::{Parser, Generator};
//! 
//! let structure = r#"
//! my-project/
//! ├── src/
//! │   ├── main.rs
//! │   └── lib.rs
//! └── Cargo.toml
//! "#;
//! 
//! let parsed = Parser::new().parse(structure)?;
//! Generator::new().generate(&parsed, "./output")?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
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


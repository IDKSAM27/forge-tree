use thiserror::Error;

pub type Result<T> = std::result::Result<T, ForgeTreeError>;

#[derive(Error, Debug)]
pub enum ForgeTreeError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Template error: {0}")]
    Template(#[from] handlebars::RenderError),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_yaml::Error),

    #[error("Invalid path: {0}")]
    InvalidPath(String),

    #[error("File already exists: {0}")]
    FileExists(String),

    #[error("Template not found: {0}")]
    TemplateNotFound(String),
}


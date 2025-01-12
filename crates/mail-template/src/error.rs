#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Template error: {0}")]
    TemplateError(#[from] tera::Error),
    #[error("Invalid template: {0}")]
    InvalidTemplate(#[from] serde_json::Error),
    #[error("Serialization error: {0}")]
    SerializationError(String),
}

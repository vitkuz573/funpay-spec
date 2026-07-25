use thiserror::Error;

#[derive(Debug, Error)]
pub enum SpecError {
    #[error("YAML parse error: {0}")]
    Yaml(#[from] serde_yaml::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("drift detected: {0}")]
    Drift(String),
    #[error("invalid selector: {0}")]
    InvalidSelector(String),
    #[error("entity not found: {0}")]
    EntityNotFound(String),
    #[error("field not found in entity {entity}: {field}")]
    FieldNotFound { entity: String, field: String },
    #[error("transform not found: {0}")]
    TransformNotFound(String),
}

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
}

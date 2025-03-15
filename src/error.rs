use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum SportError {
    #[error("Team not found: {0}")]
    TeamNotFound(String),
    #[error("Game not found: {0}")]
    GameNotFound(String),
    #[error("Player not found: {0}")]
    PlayerNotFound(String),
    #[error("Invalid date format: {0}")]
    DateError(String),
    #[error("Failed to fetch data: {0}")]
    FetchError(String),
}

impl From<anyhow::Error> for SportError {
    fn from(err: anyhow::Error) -> Self {
        SportError::FetchError(err.to_string())
    }
}

impl From<serde_json::Error> for SportError {
    fn from(err: serde_json::Error) -> Self {
        SportError::FetchError(format!("JSON error: {}", err))
    }
}

impl From<reqwest::Error> for SportError {
    fn from(err: reqwest::Error) -> Self {
        SportError::FetchError(format!("Network error: {}", err))
    }
}

impl From<std::io::Error> for SportError {
    fn from(err: std::io::Error) -> Self {
        SportError::FetchError(format!("IO error: {}", err))
    }
}

pub type Result<T> = std::result::Result<T, SportError>; 
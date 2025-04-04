use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] tokio_postgres::Error),
    
    #[error("HTTP client error: {0}")]
    HttpClient(#[from] reqwest::Error),
    
    #[error("JSON serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Environment variable error: {0}")]
    Env(#[from] std::env::VarError),
    
    #[error("AI model error: {0}")]
    AIModel(String),
    
    #[error("SQL execution error: {0}")]
    SqlExecution(String),
    
    #[error("Generic error: {0}")]
    Generic(String),
}

pub type Result<T> = std::result::Result<T, AppError>;
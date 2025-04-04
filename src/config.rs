use dotenv::dotenv;
use std::env;
use crate::error::{AppError, Result};

#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub gemini_api_key: String,
    pub gemini_api_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenv().ok();
        
        let database_url = env::var("DATABASE_URL")?;
        let gemini_api_key = env::var("GEMINI_API_KEY")?;
        let gemini_api_url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent?key={}",
            gemini_api_key
        );
        
        Ok(Self {
            database_url,
            gemini_api_key,
            gemini_api_url,
        })
    }
}
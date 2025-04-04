use reqwest::Client as HttpClient;
use async_trait::async_trait;
use std::sync::Arc;

use crate::config::Config;
use crate::models::order::{GeminiRequest, GeminiResponse};
use crate::error::{AppError, Result};

#[async_trait]
pub trait AIModel {
    async fn generate_sql(&self, prompt: &str) -> Result<String>;
    async fn interpret_results(&self, query: &str, results: &str) -> Result<String>;
}

pub struct GeminiModel {
    http_client: HttpClient,
    config: Arc<Config>,
}

impl GeminiModel {
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            http_client: HttpClient::new(),
            config,
        }
    }
    
    async fn send_request(&self, prompt: &str) -> Result<String> {
        let request = GeminiRequest::new(prompt);
        
        let response = self.http_client
            .post(&self.config.gemini_api_url)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;
            
        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(AppError::AIModel(format!("Gemini API error: {}", error_text)));
        }
            
        let gemini_response: GeminiResponse = response.json().await?;
        
        if gemini_response.candidates.is_empty() {
            return Err(AppError::AIModel("No response from Gemini".to_string()));
        }
        
        if gemini_response.candidates[0].content.parts.is_empty() {
            return Err(AppError::AIModel("Empty response from Gemini".to_string()));
        }
        
        Ok(gemini_response.candidates[0].content.parts[0].text.clone())
    }
}

#[async_trait]
impl AIModel for GeminiModel {
    async fn generate_sql(&self, prompt: &str) -> Result<String> {
        let ai_prompt = format!(
            "You are a SQL specialist. Based on the given question on natural language, \
            generate a SQL query valid for postgres that gather data that might answer the user question. \
            Return ONLY the generated SQL query, with no additional explanation or comments.\n\n\
            Available table: orders_data\n\
            Available columns: {}\n\n\
            Question: {}",
            prompt.split("\n").next().unwrap_or(""), 
            prompt.split("\n").nth(1).unwrap_or("")   
        );
        
        let response = self.send_request(&ai_prompt).await?;
        
        
        let clean_sql = response
            .trim()
            .trim_start_matches("```sql")
            .trim_end_matches("```")
            .trim();
            
        Ok(clean_sql.to_string())
    }
    
    async fn interpret_results(&self, query: &str, results: &str) -> Result<String> {
        let ai_prompt = format!(
            "You are a useful assistant that interprets SQL query results. \
            Based on the following SQL query and on the obtained results, \
            provide a friendly and informative portuguese answer that explains the results.\n\n\
            Query SQL: {}\n\n\
            Resultados: {}\n\n\
            Provide a clearly and concise answer that a non-tech user would understand.",
            query, results
        );
        
        self.send_request(&ai_prompt).await
    }
}
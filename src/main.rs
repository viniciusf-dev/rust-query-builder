mod config;
mod error;
mod models;
mod db;
mod ai;
mod services;

use std::io::{self, Write};
use std::sync::Arc;

use config::Config;
use db::connection::DbConnection;
use db::repository::PostgresRepository;
use ai::gemini::GeminiModel;
use services::query_service::QueryService;
use error::Result;

#[tokio::main]
async fn main() -> Result<()> {
    
    let config = Arc::new(Config::from_env()?);
    println!("Configuration loaded.");
    
    
    let db_connection = DbConnection::new(&config).await?;
    let client = Arc::new(db_connection.client().clone());
    let repository = Arc::new(PostgresRepository::new(client));
    println!("Connection with Postgres established.");
    
   
    let ai_model = Arc::new(GeminiModel::new(config.clone()));
    println!("LLM initialized.");
    
    
    let query_service = QueryService::new(ai_model, repository);
    println!("Query service initialized.");
    
    println!("\nWelcome to our SQL query builder!");
    println!("Aks a question (or type 'exit' para to finish your session):\n");
    
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input)?;
        
        let user_input = user_input.trim();
        
        if user_input.to_lowercase() == "exit" {
            println!("Encerrando o programa. Closing session!");
            break;
        }
        
        match query_service.process_query(user_input).await {
            Ok(response) => println!("\n{}\n", response),
            Err(e) => println!("\nErro: {}\n", e),
        }
    }
    
    Ok(())
}
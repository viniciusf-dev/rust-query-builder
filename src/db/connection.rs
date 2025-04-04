use tokio_postgres::{Client, NoTls};
use crate::error::{AppError, Result};
use crate::config::Config;

pub struct DbConnection {
    client: Client,
}

impl DbConnection {
    pub async fn new(config: &Config) -> Result<Self> {
        let (client, connection) = tokio_postgres::connect(&config.database_url, NoTls).await?;
        
        
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Database connection error: {}", e);
            }
        });
        
        Ok(Self { client })
    }
    
    pub fn client(&self) -> &Client {
        &self.client
    }
}
use std::sync::Arc;
use tokio_postgres::{Row, Client};
use crate::error::{AppError, Result};
use async_trait::async_trait;

#[async_trait]
pub trait Repository {
    async fn execute_query(&self, query: &str) -> Result<Vec<Row>>;
    async fn get_table_schema(&self, table_name: &str) -> Result<String>;
}

pub struct PostgresRepository {
    client: Arc<Client>,
}

impl PostgresRepository {
    pub fn new(client: Arc<Client>) -> Self {
        Self { client }
    }
}

#[async_trait]
impl Repository for PostgresRepository {
    async fn execute_query(&self, query: &str) -> Result<Vec<Row>> {
        let rows = self.client.query(query, &[]).await
            .map_err(|e| AppError::SqlExecution(e.to_string()))?;
        Ok(rows)
    }
    
    async fn get_table_schema(&self, table_name: &str) -> Result<String> {
        let query = format!(
            "SELECT column_name, data_type 
             FROM information_schema.columns 
             WHERE table_name = '{}'
             ORDER BY ordinal_position",
            table_name
        );
        
        let rows = self.client.query(&query, &[]).await?;
        
        let schema = rows.iter()
            .map(|row| {
                let column_name: String = row.get("column_name");
                let data_type: String = row.get("data_type");
                format!("{} ({})", column_name, data_type)
            })
            .collect::<Vec<String>>()
            .join(", ");
            
        Ok(schema)
    }
}
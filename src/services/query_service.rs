use std::sync::Arc;
use tokio_postgres::Row;

use crate::ai::gemini::AIModel;
use crate::db::repository::Repository;
use crate::error::{AppError, Result};

pub struct QueryService<A: AIModel, R: Repository> {
    ai_model: Arc<A>,
    repository: Arc<R>,
}

impl<A: AIModel, R: Repository> QueryService<A, R> {
    pub fn new(ai_model: Arc<A>, repository: Arc<R>) -> Self {
        Self { ai_model, repository }
    }
    
    pub async fn process_query(&self, user_query: &str) -> Result<String> {
        
        let table_schema = self.repository.get_table_schema("orders_data").await?;
        
        
        let full_prompt = format!("{}\n{}", table_schema, user_query);
        
       
        println!("Generating SQL query...");
        let sql_query = self.ai_model.generate_sql(&full_prompt).await?;
        println!("SQL generated: {}", sql_query);
        
        
        println!("Running query on Postgres...");
        let rows = self.repository.execute_query(&sql_query).await?;
        
        
        let results = self.format_results(&rows)?;
        println!("Results: {}", results);
        
        
        println!("Interpreting results...");
        let response = self.ai_model.interpret_results(&sql_query, &results).await?;
        
        Ok(response)
    }
    
    fn format_results(&self, rows: &[Row]) -> Result<String> {
        if rows.is_empty() {
            return Ok("No result found.".to_string());
        }
        
        
        let columns = rows[0].columns();
        let column_names: Vec<&str> = columns.iter().map(|c| c.name()).collect();
        
        
        let mut results = String::new();
        results.push_str(&column_names.join(" | "));
        results.push_str("\n");
        results.push_str(&"-".repeat(results.len()));
        results.push_str("\n");
        
        
        for row in rows {
            let mut row_values = Vec::new();
            
            for (i, column) in columns.iter().enumerate() {
                let value = match column.type_().name() {
                    "int4" => row.get::<_, i32>(i).to_string(),
                    "float8" => row.get::<_, f64>(i).to_string(),
                    "date" => row.get::<_, chrono::NaiveDate>(i).to_string(),
                    "varchar" | "text" => row.get::<_, String>(i),
                    _ => "Unknow type".to_string(),
                };
                row_values.push(value);
            }
            
            results.push_str(&row_values.join(" | "));
            results.push_str("\n");
        }
        
        Ok(results)
    }
}
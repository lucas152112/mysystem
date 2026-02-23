use sqlx::{mysql::MySqlPoolOptions, MySqlPool};
use tracing::info;

pub struct DatabasePool {
    pub pool: MySqlPool,
}

impl DatabasePool {
    pub async fn connect(url: &str) -> Result<Self, sqlx::Error> {
        info!("Connecting to database: {}", url);
        
        let pool = MySqlPoolOptions::new()
            .max_connections(5)
            .connect(url)
            .await?;
        
        info!("Database connection established");
        
        Ok(Self { pool })
    }
    
    pub async fn run_migrations(&self) -> Result<(), sqlx::Error> {
        info!("Running database migrations...");
        
        // 對於 MySQL，我們需要手動執行遷移腳本
        let migration_sql = include_str!("../migrations/20250221000000_create_tables_mysql.sql");
        
        sqlx::query(migration_sql)
            .execute(&self.pool)
            .await?;
        
        info!("Migrations completed successfully");
        Ok(())
    }
    
    pub fn get_pool(&self) -> &MySqlPool {
        &self.pool
    }
}
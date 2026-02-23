mod api;
mod models;
mod database;
mod error;

use axum::{Router, Extension};
use std::net::SocketAddr;
use tracing_subscriber;

use database::DatabasePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日誌
    tracing_subscriber::fmt::init();

    // 加載環境變量
    dotenv::dotenv().ok();
    
    // 連接數據庫
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "mysql://user:password@localhost:3306/mysystem".to_string());
    
    let pool = DatabasePool::connect(&database_url).await?;
    tracing::info!("Database connected");

    // 運行數據庫遷移
    pool.run_migrations().await?;

    // 創建API路由
    let app = Router::new()
        .nest("/api", api::routes())
        .layer(Extension(pool));

    // 啟動服務器
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Server listening on {}", addr);
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
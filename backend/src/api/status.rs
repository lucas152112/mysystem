use axum::{
    extract::State,
    routing::get,
    Json, Router,
};
use crate::{database::DatabasePool, models, error::Result};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(get_all_status))
        .route("/lights", get(get_status_lights))
}

// 獲取所有功能狀態
async fn get_all_status(
    State(pool): State<DatabasePool>,
) -> Result<Json<Vec<models::Function>>> {
    let functions = sqlx::query_as!(
        models::Function,
        r#"
        SELECT id, name, description, command, 
               args as "args!: Vec<String>", 
               timeout, status as "status!: models::FunctionStatus",
               last_executed, last_status as "last_status: Option<models::ExecutionResult>",
               created_at, updated_at
        FROM functions
        ORDER BY name
        "#
    )
    .fetch_all(pool.get_pool())
    .await?;

    Ok(Json(functions))
}

// 獲取狀態燈號
async fn get_status_lights(
    State(pool): State<DatabasePool>,
) -> Result<Json<Vec<models::StatusLight>>> {
    let functions = sqlx::query_as!(
        models::Function,
        r#"
        SELECT id, name, description, command, 
               args as "args!: Vec<String>", 
               timeout, status as "status!: models::FunctionStatus",
               last_executed, last_status as "last_status: Option<models::ExecutionResult>",
               created_at, updated_at
        FROM functions
        ORDER BY name
        "#
    )
    .fetch_all(pool.get_pool())
    .await?;

    let lights = functions.into_iter().map(|function| {
        let color = match function.status {
            models::FunctionStatus::Running => "green",
            models::FunctionStatus::Stopped => "yellow",
            models::FunctionStatus::Error => "red",
        };

        models::StatusLight {
            function_id: function.id,
            status: function.status,
            color: color.to_string(),
            last_executed: function.last_executed,
            last_status: function.last_status,
        }
    }).collect();

    Ok(Json(lights))
}
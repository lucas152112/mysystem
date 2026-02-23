use axum::{
    extract::{Path, State},
    routing::{get, post, put, delete},
    Json, Router,
};
use uuid::Uuid;
use crate::{database::DatabasePool, models, error::{AppError, Result}};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(list_functions).post(create_function))
        .route("/:id", get(get_function).put(update_function).delete(delete_function))
        .route("/:id/run", post(run_function))
        .route("/:id/logs", get(get_execution_logs))
}

// 獲取所有功能
async fn list_functions(
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
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(pool.get_pool())
    .await?;

    Ok(Json(functions))
}

// 創建新功能
async fn create_function(
    State(pool): State<DatabasePool>,
    Json(input): Json<models::CreateFunctionRequest>,
) -> Result<Json<models::Function>> {
    let id = Uuid::new_v4();
    let args = input.args.unwrap_or_default();
    let timeout = input.timeout.unwrap_or(30);
    
    let function = sqlx::query_as!(
        models::Function,
        r#"
        INSERT INTO functions (id, name, description, command, args, timeout)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, name, description, command, 
                  args as "args!: Vec<String>", 
                  timeout, status as "status!: models::FunctionStatus",
                  last_executed, last_status as "last_status: Option<models::ExecutionResult>",
                  created_at, updated_at
        "#,
        id, input.name, input.description, input.command,
        args as _, timeout
    )
    .fetch_one(pool.get_pool())
    .await?;

    Ok(Json(function))
}

// 獲取單個功能
async fn get_function(
    State(pool): State<DatabasePool>,
    Path(id): Path<Uuid>,
) -> Result<Json<models::Function>> {
    let function = sqlx::query_as!(
        models::Function,
        r#"
        SELECT id, name, description, command, 
               args as "args!: Vec<String>", 
               timeout, status as "status!: models::FunctionStatus",
               last_executed, last_status as "last_status: Option<models::ExecutionResult>",
               created_at, updated_at
        FROM functions
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool.get_pool())
    .await?
    .ok_or_else(|| AppError::FunctionNotFound(id.to_string()))?;

    Ok(Json(function))
}

// 更新功能
async fn update_function(
    State(pool): State<DatabasePool>,
    Path(id): Path<Uuid>,
    Json(input): Json<models::UpdateFunctionRequest>,
) -> Result<Json<models::Function>> {
    let function = sqlx::query_as!(
        models::Function,
        r#"
        UPDATE functions
        SET 
            name = COALESCE($2, name),
            description = COALESCE($3, description),
            command = COALESCE($4, command),
            args = COALESCE($5, args),
            timeout = COALESCE($6, timeout),
            updated_at = NOW()
        WHERE id = $1
        RETURNING id, name, description, command, 
                  args as "args!: Vec<String>", 
                  timeout, status as "status!: models::FunctionStatus",
                  last_executed, last_status as "last_status: Option<models::ExecutionResult>",
                  created_at, updated_at
        "#,
        id, input.name, input.description, input.command,
        input.args as _, input.timeout
    )
    .fetch_optional(pool.get_pool())
    .await?
    .ok_or_else(|| AppError::FunctionNotFound(id.to_string()))?;

    Ok(Json(function))
}

// 刪除功能
async fn delete_function(
    State(pool): State<DatabasePool>,
    Path(id): Path<Uuid>,
) -> Result<Json<()>> {
    let result = sqlx::query!(
        "DELETE FROM functions WHERE id = $1",
        id
    )
    .execute(pool.get_pool())
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::FunctionNotFound(id.to_string()));
    }

    Ok(Json(()))
}

// 執行功能（佔位符實現）
async fn run_function(
    State(pool): State<DatabasePool>,
    Path(id): Path<Uuid>,
    Json(input): Json<models::ExecuteFunctionRequest>,
) -> Result<Json<models::Execution>> {
    // 這裡將實現實際的命令執行邏輯
    // 目前返回一個佔位符執行記錄
    
    let execution_id = Uuid::new_v4();
    let start_time = chrono::Utc::now();
    let result = models::ExecutionResult::Success;
    
    let execution = models::Execution {
        id: execution_id.to_string(),
        function_id: id.to_string(),
        start_time,
        end_time: Some(start_time),
        exit_code: Some(0),
        output: Some("Execution placeholder".to_string()),
        error: None,
        result,
    };

    Ok(Json(execution))
}

// 獲取執行記錄
async fn get_execution_logs(
    State(pool): State<DatabasePool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Vec<models::Execution>>> {
    let executions = sqlx::query_as!(
        models::Execution,
        r#"
        SELECT id, function_id, start_time, end_time, 
               exit_code, output, error, 
               result as "result!: models::ExecutionResult"
        FROM executions
        WHERE function_id = $1
        ORDER BY start_time DESC
        LIMIT 100
        "#,
        id
    )
    .fetch_all(pool.get_pool())
    .await?;

    Ok(Json(executions))
}
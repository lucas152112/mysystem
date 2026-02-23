use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// 功能狀態枚舉 - MySQL ENUM 映射
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "function_status", rename_all = "lowercase")]
pub enum FunctionStatus {
    Running,   // 運行中
    Stopped,   // 已停止
    Error,     // 錯誤
}

// 執行結果枚舉 - MySQL ENUM 映射
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "execution_result", rename_all = "lowercase")]
pub enum ExecutionResult {
    Success,   // 成功
    Failed,    // 失敗
    Timeout,   // 超時
    Stopped,   // 手動停止
}

// 功能定義
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Function {
    pub id: String,               // MySQL 使用 CHAR(36) 存儲 UUID
    pub name: String,
    pub description: Option<String>,
    pub command: String,          // 執行的命令/腳本
    pub args: Vec<String>,        // 命令參數
    pub timeout: i32,             // 超時時間(秒)
    pub status: FunctionStatus,   // 當前狀態
    pub last_executed: Option<DateTime<Utc>>,
    pub last_status: Option<ExecutionResult>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// 執行記錄
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Execution {
    pub id: String,              // MySQL 使用 CHAR(36) 存儲 UUID
    pub function_id: String,     // MySQL 使用 CHAR(36) 存儲 UUID
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub exit_code: Option<i32>,
    pub output: Option<String>,   // 標準輸出
    pub error: Option<String>,    // 錯誤輸出
    pub result: ExecutionResult,
}

// 創建功能的請求
#[derive(Debug, Clone, Deserialize)]
pub struct CreateFunctionRequest {
    pub name: String,
    pub description: Option<String>,
    pub command: String,
    pub args: Option<Vec<String>>,
    pub timeout: Option<i32>,
}

// 更新功能的請求
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateFunctionRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub command: Option<String>,
    pub args: Option<Vec<String>>,
    pub timeout: Option<i32>,
}

// 執行功能的請求
#[derive(Debug, Clone, Deserialize)]
pub struct ExecuteFunctionRequest {
    pub args: Option<Vec<String>>,
    pub timeout: Option<i32>,
}

// 狀態燈號響應
#[derive(Debug, Clone, Serialize)]
pub struct StatusLight {
    pub function_id: String,
    pub status: FunctionStatus,
    pub color: String,           // "green", "red", "yellow"
    pub last_executed: Option<DateTime<Utc>>,
    pub last_status: Option<ExecutionResult>,
}

// 為 Function 實現轉換方法
impl Function {
    pub fn new_id() -> String {
        Uuid::new_v4().to_string()
    }
}

// 為 Execution 實現轉換方法
impl Execution {
    pub fn new_id() -> String {
        Uuid::new_v4().to_string()
    }
}
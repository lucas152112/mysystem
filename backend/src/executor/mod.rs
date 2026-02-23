use tokio::process::Command;
use std::time::Duration;
use tokio::time::timeout;
use tracing::{info, warn, error};
use crate::models::{ExecutionResult, Execution};
use chrono::Utc;

pub struct CommandExecutor;

impl CommandExecutor {
    /// 執行系統命令
    pub async fn execute(
        command: &str,
        args: &[String],
        timeout_seconds: i32,
    ) -> Result<Execution, String> {
        let start_time = Utc::now();
        let execution_id = crate::models::Execution::new_id();
        
        info!("Executing command: {} {:?}", command, args);
        
        let mut cmd = Command::new(command);
        cmd.args(args);
        
        // 設置超時
        let duration = Duration::from_secs(timeout_seconds as u64);
        
        match timeout(duration, cmd.output()).await {
            Ok(Ok(output)) => {
                let end_time = Utc::now();
                let exit_code = output.status.code();
                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                
                info!("Command executed successfully: exit_code={:?}", exit_code);
                
                let result = if output.status.success() {
                    ExecutionResult::Success
                } else {
                    ExecutionResult::Failed
                };
                
                Ok(Execution {
                    id: execution_id,
                    function_id: "placeholder".to_string(),
                    start_time,
                    end_time: Some(end_time),
                    exit_code,
                    output: Some(stdout),
                    error: if stderr.is_empty() { None } else { Some(stderr) },
                    result,
                })
            }
            Ok(Err(e)) => {
                error!("Command execution failed: {}", e);
                Err(format!("Command execution failed: {}", e))
            }
            Err(_) => {
                warn!("Command timed out after {} seconds", timeout_seconds);
                Err(format!("Command timed out after {} seconds", timeout_seconds))
            }
        }
    }
    
    /// 安全執行命令（限制權限）
    pub async fn execute_safe(
        command: &str,
        args: &[String],
        timeout_seconds: i32,
        allowed_commands: &[&str],
    ) -> Result<Execution, String> {
        // 檢查命令是否在白名單中
        if !allowed_commands.contains(&command) {
            return Err(format!("Command '{}' is not in allowed list", command));
        }
        
        // 檢查危險命令
        let dangerous = ["rm", "dd", "mkfs", "fdisk", "shutdown", "reboot"];
        if dangerous.contains(&command) {
            warn!("Attempted to execute dangerous command: {}", command);
            return Err(format!("Dangerous command '{}' is blocked", command));
        }
        
        Self::execute(command, args, timeout_seconds).await
    }
    
    /// 執行 shell 腳本
    pub async fn execute_shell(
        script: &str,
        timeout_seconds: i32,
    ) -> Result<Execution, String> {
        let start_time = Utc::now();
        let execution_id = crate::models::Execution::new_id();
        
        info!("Executing shell script: {}", script);
        
        let mut cmd = Command::new("bash");
        cmd.arg("-c").arg(script);
        
        let duration = Duration::from_secs(timeout_seconds as u64);
        
        match timeout(duration, cmd.output()).await {
            Ok(Ok(output)) => {
                let end_time = Utc::now();
                let exit_code = output.status.code();
                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                
                info!("Shell script executed successfully: exit_code={:?}", exit_code);
                
                let result = if output.status.success() {
                    ExecutionResult::Success
                } else {
                    ExecutionResult::Failed
                };
                
                Ok(Execution {
                    id: execution_id,
                    function_id: "placeholder".to_string(),
                    start_time,
                    end_time: Some(end_time),
                    exit_code,
                    output: Some(stdout),
                    error: if stderr.is_empty() { None } else { Some(stderr) },
                    result,
                })
            }
            Ok(Err(e)) => {
                error!("Shell script execution failed: {}", e);
                Err(format!("Shell script execution failed: {}", e))
            }
            Err(_) => {
                warn!("Shell script timed out after {} seconds", timeout_seconds);
                Err(format!("Shell script timed out after {} seconds", timeout_seconds))
            }
        }
    }
}
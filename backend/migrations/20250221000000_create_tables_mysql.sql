-- MySQL 資料表結構
-- 注意：MySQL 8.0+ 支援 JSON 類型

-- 功能定義表
CREATE TABLE functions (
    id CHAR(36) PRIMARY KEY DEFAULT (UUID()),
    name VARCHAR(100) NOT NULL,
    description TEXT,
    command TEXT NOT NULL,
    args JSON DEFAULT (JSON_ARRAY()),
    timeout INT DEFAULT 30,
    status ENUM('running', 'stopped', 'error') DEFAULT 'stopped',
    last_executed TIMESTAMP NULL,
    last_status ENUM('success', 'failed', 'timeout', 'stopped') NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    
    UNIQUE KEY functions_name_unique (name)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- 執行記錄表
CREATE TABLE executions (
    id CHAR(36) PRIMARY KEY DEFAULT (UUID()),
    function_id CHAR(36) NOT NULL,
    start_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    end_time TIMESTAMP NULL,
    exit_code INT NULL,
    output TEXT NULL,
    error TEXT NULL,
    result ENUM('success', 'failed', 'timeout', 'stopped') NOT NULL,
    
    INDEX idx_executions_function_id (function_id),
    INDEX idx_executions_start_time (start_time),
    FOREIGN KEY (function_id) REFERENCES functions(id) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- 系統設定表
CREATE TABLE settings (
    `key` VARCHAR(100) PRIMARY KEY,
    value JSON NOT NULL,
    description TEXT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- 插入默認設定
INSERT INTO settings (`key`, value, description) VALUES
    ('system.name', '"mysystem"', '系統名稱'),
    ('max_concurrent_executions', '10', '最大併發執行數'),
    ('default_timeout', '30', '默認超時時間(秒)');
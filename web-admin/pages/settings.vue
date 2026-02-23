<template>
  <div>
    <!-- 頁面標題 -->
    <div style="margin-bottom: 24px;">
      <h1 style="font-size: 24px; font-weight: 600; margin-bottom: 8px;">系統設定</h1>
      <p style="color: #666;">配置系統參數和安全設定</p>
    </div>
    
    <!-- 設定卡片 -->
    <a-row :gutter="[16, 16]">
      <!-- API 設定 -->
      <a-col :span="24">
        <a-card title="API 設定" style="margin-bottom: 16px;">
          <a-form
            ref="apiFormRef"
            :model="apiSettings"
            layout="vertical"
            @finish="saveApiSettings"
          >
            <a-row :gutter="16">
              <a-col :span="12">
                <a-form-item label="API 基礎 URL" name="apiBaseUrl">
                  <a-input
                    v-model:value="apiSettings.apiBaseUrl"
                    placeholder="例如：http://localhost:3000/api"
                  />
                </a-form-item>
              </a-col>
              <a-col :span="12">
                <a-form-item label="WebSocket URL" name="websocketUrl">
                  <a-input
                    v-model:value="apiSettings.websocketUrl"
                    placeholder="例如：ws://localhost:3000/api/ws"
                  />
                </a-form-item>
              </a-col>
            </a-row>
            
            <a-form-item>
              <a-space>
                <a-button type="primary" html-type="submit" :loading="saving">
                  儲存 API 設定
                </a-button>
                <a-button @click="testApiConnection">
                  測試連接
                </a-button>
              </a-space>
            </a-form-item>
          </a-form>
        </a-card>
      </a-col>
      
      <!-- 安全設定 -->
      <a-col :span="12">
        <a-card title="安全設定">
          <a-form :model="securitySettings" layout="vertical">
            <a-form-item label="允許的命令白名單">
              <a-textarea
                v-model:value="securitySettings.allowedCommands"
                placeholder="每行一個命令"
                :rows="6"
              />
              <template #extra>
                <span style="font-size: 12px; color: #999;">
                  只允許列表中的命令執行，留空表示不限制
                </span>
              </template>
            </a-form-item>
            
            <a-form-item label="最大執行時間（秒）">
              <a-input-number
                v-model:value="securitySettings.maxExecutionTime"
                :min="1"
                :max="300"
                style="width: 100%"
              />
            </a-form-item>
            
            <a-form-item label="最大併發執行數">
              <a-input-number
                v-model:value="securitySettings.maxConcurrentExecutions"
                :min="1"
                :max="20"
                style="width: 100%"
              />
            </a-form-item>
            
            <a-form-item>
              <a-button type="primary" @click="saveSecuritySettings" :loading="saving">
                儲存安全設定
              </a-button>
            </a-form-item>
          </a-form>
        </a-card>
      </a-col>
      
      <!-- 系統資訊 -->
      <a-col :span="12">
        <a-card title="系統資訊">
          <a-descriptions :column="1" size="small">
            <a-descriptions-item label="系統版本">
              {{ systemInfo.version }}
            </a-descriptions-item>
            <a-descriptions-item label="最後啟動時間">
              {{ formatTime(systemInfo.lastStartTime) }}
            </a-descriptions-item>
            <a-descriptions-item label="運行時長">
              {{ calculateUptime(systemInfo.lastStartTime) }}
            </a-descriptions-item>
            <a-descriptions-item label="總功能數">
              {{ systemInfo.totalFunctions }}
            </a-descriptions-item>
            <a-descriptions-item label="總執行次數">
              {{ systemInfo.totalExecutions }}
            </a-descriptions-item>
          </a-descriptions>
          
          <div style="margin-top: 16px;">
            <a-button-group style="width: 100%">
              <a-button @click="refreshSystemInfo">
                刷新資訊
              </a-button>
              <a-button @click="exportSystemInfo">
                導出資訊
              </a-button>
              <a-button @click="clearSystemCache">
                清除緩存
              </a-button>
            </a-button-group>
          </div>
        </a-card>
      </a-col>
      
      <!-- 數據庫管理 -->
      <a-col :span="24">
        <a-card title="數據庫管理">
          <a-space :size="16" style="margin-bottom: 16px;">
            <a-button @click="backupDatabase">
              備份數據庫
            </a-button>
            <a-button @click="restoreDatabase" :disabled="!hasBackup">
              恢復數據庫
            </a-button>
            <a-button @click="migrateDatabase">
              執行遷移
            </a-button>
            <a-button danger @click="clearDatabase">
              清空數據庫
            </a-button>
          </a-space>
          
          <a-alert
            message="數據庫操作警告"
            description="這些操作可能會影響系統運行，請謹慎操作"
            type="warning"
            show-icon
          />
        </a-card>
      </a-col>
      
      <!-- 高級設定 -->
      <a-col :span="24">
        <a-card title="高級設定">
          <a-form :model="advancedSettings" layout="vertical">
            <a-row :gutter="16">
              <a-col :span="8">
                <a-form-item label="日誌級別">
                  <a-select
                    v-model:value="advancedSettings.logLevel"
                    style="width: 100%"
                  >
                    <a-select-option value="debug">Debug</a-select-option>
                    <a-select-option value="info">Info</a-select-option>
                    <a-select-option value="warn">Warn</a-select-option>
                    <a-select-option value="error">Error</a-select-option>
                  </a-select>
                </a-form-item>
              </a-col>
              
              <a-col :span="8">
                <a-form-item label="自動刷新間隔（秒）">
                  <a-input-number
                    v-model:value="advancedSettings.autoRefreshInterval"
                    :min="5"
                    :max="300"
                    style="width: 100%"
                  />
                </a-form-item>
              </a-col>
              
              <a-col :span="8">
                <a-form-item label="日誌保留天數">
                  <a-input-number
                    v-model:value="advancedSettings.logRetentionDays"
                    :min="1"
                    :max="365"
                    style="width: 100%"
                  />
                </a-form-item>
              </a-col>
            </a-row>
            
            <a-form-item label="通知設定">
              <a-checkbox-group v-model:value="advancedSettings.notifications">
                <a-checkbox value="email">郵件通知</a-checkbox>
                <a-checkbox value="webhook">WebHook通知</a-checkbox>
                <a-checkbox value="browser">瀏覽器通知</a-checkbox>
              </a-checkbox-group>
            </a-form-item>
            
            <a-form-item>
              <a-button type="primary" @click="saveAdvancedSettings" :loading="saving">
                儲存高級設定
              </a-button>
            </a-form-item>
          </a-form>
        </a-card>
      </a-col>
    </a-row>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { message, type FormInstance } from 'ant-design-vue'
import { setApiBaseUrl } from '~/utils/api'

// 表單引用
const apiFormRef = ref<FormInstance>()

// 設定狀態
const saving = ref(false)
const hasBackup = ref(false)

// API 設定
const apiSettings = ref({
  apiBaseUrl: 'http://localhost:3000/api',
  websocketUrl: 'ws://localhost:3000/api/ws',
})

// 安全設定
const securitySettings = ref({
  allowedCommands: '',
  maxExecutionTime: 30,
  maxConcurrentExecutions: 10,
})

// 系統資訊
const systemInfo = ref({
  version: '1.0.0',
  lastStartTime: new Date().toISOString(),
  totalFunctions: 0,
  totalExecutions: 0,
})

// 高級設定
const advancedSettings = ref({
  logLevel: 'info',
  autoRefreshInterval: 10,
  logRetentionDays: 30,
  notifications: ['browser'],
})

// 初始化
onMounted(() => {
  loadSettings()
  loadSystemInfo()
})

// 載入設定
const loadSettings = () => {
  // 從 localStorage 載入設定
  const savedApiSettings = localStorage.getItem('apiSettings')
  if (savedApiSettings) {
    apiSettings.value = JSON.parse(savedApiSettings)
  }
  
  const savedSecuritySettings = localStorage.getItem('securitySettings')
  if (savedSecuritySettings) {
    securitySettings.value = JSON.parse(savedSecuritySettings)
  }
  
  const savedAdvancedSettings = localStorage.getItem('advancedSettings')
  if (savedAdvancedSettings) {
    advancedSettings.value = JSON.parse(savedAdvancedSettings)
  }
}

// 載入系統資訊
const loadSystemInfo = async () => {
  try {
    // 這裡可以調用 API 獲取系統資訊
    // 目前使用模擬數據
    systemInfo.value = {
      version: '1.0.0',
      lastStartTime: new Date().toISOString(),
      totalFunctions: 12,
      totalExecutions: 45,
    }
  } catch (error) {
    message.error('載入系統資訊失敗')
  }
}

// 儲存 API 設定
const saveApiSettings = async () => {
  saving.value = true
  try {
    localStorage.setItem('apiSettings', JSON.stringify(apiSettings.value))
    
    // 更新 API 基礎 URL
    setApiBaseUrl(apiSettings.value.apiBaseUrl)
    
    message.success('API 設定已儲存')
  } catch (error) {
    message.error('儲存失敗')
  } finally {
    saving.value = false
  }
}

// 測試 API 連接
const testApiConnection = async () => {
  try {
    message.loading({ content: '測試連接中...', key: 'testConnection' })
    
    // 測試 API 連接
    const response = await fetch(`${apiSettings.value.apiBaseUrl}/health`)
    if (response.ok) {
      message.success({ content: 'API 連接成功', key: 'testConnection' })
    } else {
      message.error({ content: 'API 連接失敗', key: 'testConnection' })
    }
  } catch (error) {
    message.error({ content: 'API 連接失敗', key: 'testConnection' })
  }
}

// 儲存安全設定
const saveSecuritySettings = async () => {
  saving.value = true
  try {
    localStorage.setItem('securitySettings', JSON.stringify(securitySettings.value))
    message.success('安全設定已儲存')
  } catch (error) {
    message.error('儲存失敗')
  } finally {
    saving.value = false
  }
}

// 儲存高級設定
const saveAdvancedSettings = async () => {
  saving.value = true
  try {
    localStorage.setItem('advancedSettings', JSON.stringify(advancedSettings.value))
    message.success('高級設定已儲存')
  } catch (error) {
    message.error('儲存失敗')
  } finally {
    saving.value = false
  }
}

// 刷新系統資訊
const refreshSystemInfo = async () => {
  await loadSystemInfo()
  message.success('系統資訊已刷新')
}

// 導出系統資訊
const exportSystemInfo = () => {
  const data = {
    timestamp: new Date().toISOString(),
    systemInfo: systemInfo.value,
    settings: {
      api: apiSettings.value,
      security: securitySettings.value,
      advanced: advancedSettings.value,
    },
  }
  
  const json = JSON.stringify(data, null, 2)
  const blob = new Blob([json], { type: 'application/json' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = `system-export-${new Date().toISOString().slice(0, 10)}.json`
  document.body.appendChild(a)
  a.click()
  document.body.removeChild(a)
  URL.revokeObjectURL(url)
  
  message.success('系統資訊已導出')
}

// 清除系統緩存
const clearSystemCache = () => {
  localStorage.removeItem('apiSettings')
  localStorage.removeItem('securitySettings')
  localStorage.removeItem('advancedSettings')
  
  loadSettings()
  message.success('系統緩存已清除')
}

// 備份數據庫
const backupDatabase = async () => {
  try {
    message.loading({ content: '備份數據庫中...', key: 'backup' })
    
    // 這裡應該調用 API 備份數據庫
    await new Promise(resolve => setTimeout(resolve, 1000))
    
    hasBackup.value = true
    message.success({ content: '數據庫備份成功', key: 'backup' })
  } catch (error) {
    message.error({ content: '備份失敗', key: 'backup' })
  }
}

// 恢復數據庫
const restoreDatabase = async () => {
  message.info('恢復數據庫功能開發中')
}

// 執行遷移
const migrateDatabase = async () => {
  message.info('執行遷移功能開發中')
}

// 清空數據庫
const clearDatabase = () => {
  message.error('清空數據庫功能需要確認，請謹慎操作')
}

// 格式化時間
const formatTime = (timestamp: string) => {
  try {
    const date = new Date(timestamp)
    return date.toLocaleString('zh-TW', {
      year: 'numeric',
      month: 'short',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
      second: '2-digit',
    })
  } catch (error) {
    return '時間錯誤'
  }
}

// 計算運行時長
const calculateUptime = (startTime: string) => {
  try {
    const start = new Date(startTime).getTime()
    const now = Date.now()
    const diffMs = now - start
    
    const days = Math.floor(diffMs / (1000 * 60 * 60 * 24))
    const hours = Math.floor((diffMs % (1000 * 60 * 60 * 24)) / (1000 * 60 * 60))
    const minutes = Math.floor((diffMs % (1000 * 60 * 60)) / (1000 * 60))
    
    if (days > 0) {
      return `${days}天${hours}小時${minutes}分鐘`
    } else if (hours > 0) {
      return `${hours}小時${minutes}分鐘`
    } else {
      return `${minutes}分鐘`
    }
  } catch (error) {
    return '計算錯誤'
  }
}
</script>

<style scoped>
:deep(.ant-card) {
  margin-bottom: 16px;
}

:deep(.ant-card-head) {
  background: #fafafa;
}

:deep(.ant-descriptions-item-label) {
  font-weight: 500;
}
</style>
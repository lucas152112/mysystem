<template>
  <div>
    <!-- 頁面標題 -->
    <div style="margin-bottom: 24px;">
      <h1 style="font-size: 24px; font-weight: 600; margin-bottom: 8px;">狀態監控</h1>
      <p style="color: #666;">實時監控系統功能狀態和執行情況</p>
    </div>
    
    <!-- 連接狀態 -->
    <a-card style="margin-bottom: 24px;">
      <div style="display: flex; justify-content: space-between; align-items: center;">
        <div style="display: flex; align-items: center; gap: 12px;">
          <div 
            :style="{
              width: '12px',
              height: '12px',
              borderRadius: '50%',
              backgroundColor: statusStore.websocketConnected ? '#52c41a' : '#ff4d4f',
              transition: 'all 0.3s',
              animation: statusStore.websocketConnected ? 'pulse 2s infinite' : 'none'
            }"
          />
          <div>
            <h3 style="margin: 0; font-size: 16px;">實時狀態連接</h3>
            <p style="margin: 0; color: #666; font-size: 14px;">
              {{ statusStore.websocketConnected ? '已連接' : '未連接' }}
              <span v-if="statusStore.websocketError" style="color: #ff4d4f;">
                - {{ statusStore.websocketError }}
              </span>
            </p>
          </div>
        </div>
        
        <a-space :size="16">
          <a-button 
            @click="statusStore.refreshStatus"
            :loading="statusStore.loading"
          >
            手動刷新
          </a-button>
          <a-button 
            type="primary" 
            v-if="!statusStore.websocketConnected"
            @click="statusStore.connectWebSocket"
          >
            重新連接
          </a-button>
        </a-space>
      </div>
    </a-card>
    
    <!-- 狀態統計 -->
    <a-row :gutter="[16, 16]" style="margin-bottom: 24px;">
      <a-col :span="6">
        <a-statistic-card title="總功能數" :value="statusStats.total" />
      </a-col>
      <a-col :span="6">
        <a-statistic-card 
          title="正常運行" 
          :value="statusStats.green" 
          :value-style="{ color: '#52c41a' }"
          :description="`${greenPercentage}%`"
        />
      </a-col>
      <a-col :span="6">
        <a-statistic-card 
          title="已停止" 
          :value="statusStats.yellow" 
          :value-style="{ color: '#faad14' }"
          :description="`${yellowPercentage}%`"
        />
      </a-col>
      <a-col :span="6">
        <a-statistic-card 
          title="錯誤" 
          :value="statusStats.red" 
          :value-style="{ color: '#ff4d4f' }"
          :description="`${redPercentage}%`"
        />
      </a-col>
    </a-row>
    
    <!-- 狀態分佈圖 -->
    <a-card title="狀態分佈" style="margin-bottom: 24px;">
      <div style="height: 300px; display: flex; align-items: center; justify-content: center;">
        <div style="width: 100%; max-width: 600px;">
          <a-row :gutter="[8, 8]">
            <!-- 綠燈狀態 -->
            <a-col :span="24">
              <div style="display: flex; align-items: center; gap: 12px; margin-bottom: 8px;">
                <div style="width: 12px; height: 12px; border-radius: 50%; background: #52c41a;"></div>
                <span>正常運行 ({{ statusStats.green }})</span>
                <span style="margin-left: auto; color: #666;">{{ greenPercentage }}%</span>
              </div>
              <a-progress 
                :percent="greenPercentage" 
                :stroke-color="{
                  '0%': '#52c41a',
                  '100%': '#52c41a'
                }"
                :show-info="false"
              />
            </a-col>
            
            <!-- 黃燈狀態 -->
            <a-col :span="24">
              <div style="display: flex; align-items: center; gap: 12px; margin-bottom: 8px;">
                <div style="width: 12px; height: 12px; border-radius: 50%; background: #faad14;"></div>
                <span>已停止 ({{ statusStats.yellow }})</span>
                <span style="margin-left: auto; color: #666;">{{ yellowPercentage }}%</span>
              </div>
              <a-progress 
                :percent="yellowPercentage" 
                :stroke-color="{
                  '0%': '#faad14',
                  '100%': '#faad14'
                }"
                :show-info="false"
              />
            </a-col>
            
            <!-- 紅燈狀態 -->
            <a-col :span="24">
              <div style="display: flex; align-items: center; gap: 12px; margin-bottom: 8px;">
                <div style="width: 12px; height: 12px; border-radius: 50%; background: #ff4d4f;"></div>
                <span>錯誤 ({{ statusStats.red }})</span>
                <span style="margin-left: auto; color: #666;">{{ redPercentage }}%</span>
              </div>
              <a-progress 
                :percent="redPercentage" 
                :stroke-color="{
                  '0%': '#ff4d4f',
                  '100%': '#ff4d4f'
                }"
                :show-info="false"
              />
            </a-col>
          </a-row>
        </div>
      </div>
    </a-card>
    
    <!-- 實時狀態列表 -->
    <a-card title="功能狀態詳情">
      <template #extra>
        <a-space :size="8">
          <a-button size="small" @click="toggleAutoRefresh">
            {{ autoRefreshEnabled ? '停止自動刷新' : '開啟自動刷新' }}
          </a-button>
          <a-button size="small" @click="exportStatus">
            導出狀態
          </a-button>
        </a-space>
      </template>
      
      <a-table
        :dataSource="statusLights"
        :columns="columns"
        :loading="statusStore.loading"
        :pagination="{ pageSize: 10 }"
        rowKey="function_id"
      >
        <!-- 狀態列 -->
        <template #status="{ record }">
          <div style="display: flex; align-items: center; gap: 8px;">
            <div 
              :style="{
                width: '10px',
                height: '10px',
                borderRadius: '50%',
                backgroundColor: record.color,
                animation: record.status === 'running' ? 'pulse 2s infinite' : 'none'
              }"
            />
            <span>{{ getStatusText(record.status) }}</span>
          </div>
        </template>
        
        <!-- 最後執行時間列 -->
        <template #last_executed="{ record }">
          <span v-if="record.last_executed">
            {{ formatTime(record.last_executed) }}
          </span>
          <span v-else style="color: #999;">從未執行</span>
        </template>
        
        <!-- 最後狀態列 -->
        <template #last_status="{ record }">
          <span v-if="record.last_status">
            <a-tag :color="getResultColor(record.last_status)">
              {{ getResultText(record.last_status) }}
            </a-tag>
          </span>
          <span v-else style="color: #999;">-</span>
        </template>
        
        <!-- 操作列 -->
        <template #action="{ record }">
          <a-space :size="8">
            <a-button 
              size="small"
              type="link"
              @click="viewFunctionDetails(record.function_id)"
            >
              詳情
            </a-button>
            <a-button 
              size="small"
              type="link"
              @click="viewExecutionLogs(record.function_id)"
            >
              記錄
            </a-button>
          </a-space>
        </template>
      </a-table>
    </a-card>
    
    <!-- 自動刷新定時器 -->
    <a-alert
      v-if="autoRefreshEnabled"
      message="自動刷新已啟用"
      description="狀態將每10秒自動刷新一次"
      type="info"
      show-icon
      style="margin-top: 24px;"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onBeforeUnmount } from 'vue'
import { useRouter } from '#app'
import { useStatusStore } from '~/stores/status'
import { useFunctionStore } from '~/stores/functions'
import { message } from 'ant-design-vue'

const router = useRouter()
const statusStore = useStatusStore()
const functionStore = useFunctionStore()

// 自動刷新
const autoRefreshEnabled = ref(false)
let refreshTimer: NodeJS.Timeout | null = null

// 表格列定義
const columns = [
  {
    title: '功能ID',
    dataIndex: 'function_id',
    key: 'function_id',
    width: 120,
    ellipsis: true,
  },
  {
    title: '狀態',
    dataIndex: 'status',
    key: 'status',
    slots: { customRender: 'status' },
    width: 100,
  },
  {
    title: '最後執行時間',
    dataIndex: 'last_executed',
    key: 'last_executed',
    slots: { customRender: 'last_executed' },
    width: 120,
  },
  {
    title: '最後結果',
    dataIndex: 'last_status',
    key: 'last_status',
    slots: { customRender: 'last_status' },
    width: 100,
  },
  {
    title: '操作',
    key: 'action',
    slots: { customRender: 'action' },
    width: 100,
  },
]

// 計算屬性
const statusLights = computed(() => statusStore.statusLights)
const statusStats = computed(() => statusStore.statusStats)

// 百分比計算
const greenPercentage = computed(() => {
  const total = statusStats.value.total
  return total > 0 ? Math.round((statusStats.value.green / total) * 100) : 0
})

const yellowPercentage = computed(() => {
  const total = statusStats.value.total
  return total > 0 ? Math.round((statusStats.value.yellow / total) * 100) : 0
})

const redPercentage = computed(() => {
  const total = statusStats.value.total
  return total > 0 ? Math.round((statusStats.value.red / total) * 100) : 0
})

// 初始化
onMounted(async () => {
  await loadStatus()
  statusStore.connectWebSocket()
})

// 清理
onBeforeUnmount(() => {
  stopAutoRefresh()
  statusStore.disconnectWebSocket()
})

// 加載狀態
const loadStatus = async () => {
  try {
    await statusStore.fetchStatusLights()
  } catch (error) {
    message.error('加載狀態失敗')
  }
}

// 切換自動刷新
const toggleAutoRefresh = () => {
  if (autoRefreshEnabled.value) {
    stopAutoRefresh()
  } else {
    startAutoRefresh()
  }
}

// 開始自動刷新
const startAutoRefresh = () => {
  autoRefreshEnabled.value = true
  refreshTimer = setInterval(() => {
    statusStore.refreshStatus()
  }, 10000) // 10秒刷新一次
  
  message.info('自動刷新已啟用')
}

// 停止自動刷新
const stopAutoRefresh = () => {
  autoRefreshEnabled.value = false
  if (refreshTimer) {
    clearInterval(refreshTimer)
    refreshTimer = null
  }
  message.info('自動刷新已停止')
}

// 導出狀態
const exportStatus = () => {
  const data = {
    timestamp: new Date().toISOString(),
    total: statusStats.value.total,
    green: statusStats.value.green,
    yellow: statusStats.value.yellow,
    red: statusStats.value.red,
    lights: statusLights.value,
  }
  
  const json = JSON.stringify(data, null, 2)
  const blob = new Blob([json], { type: 'application/json' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = `status-export-${new Date().toISOString().slice(0, 10)}.json`
  document.body.appendChild(a)
  a.click()
  document.body.removeChild(a)
  URL.revokeObjectURL(url)
  
  message.success('狀態導出成功')
}

// 查看功能詳情
const viewFunctionDetails = (functionId: string) => {
  router.push(`/functions/${functionId}`)
}

// 查看執行記錄
const viewExecutionLogs = (functionId: string) => {
  // 可以在這裡實現跳轉到執行記錄頁面，或顯示模態框
  message.info('查看執行記錄功能開發中')
}

// 格式化時間
const formatTime = (timestamp: string) => {
  try {
    const date = new Date(timestamp)
    return date.toLocaleString('zh-TW', {
      month: 'short',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
    })
  } catch (error) {
    return '時間錯誤'
  }
}

// 狀態文本
const getStatusText = (status: string) => {
  switch (status) {
    case 'running': return '運行中'
    case 'stopped': return '已停止'
    case 'error': return '錯誤'
    default: return '未知'
  }
}

// 結果顏色映射
const getResultColor = (result: string) => {
  switch (result) {
    case 'success': return 'success'
    case 'failed': return 'error'
    case 'timeout': return 'warning'
    case 'stopped': return 'default'
    default: return 'default'
  }
}

// 結果文本
const getResultText = (result: string) => {
  switch (result) {
    case 'success': return '成功'
    case 'failed': return '失敗'
    case 'timeout': return '超時'
    case 'stopped': return '停止'
    default: return '未知'
  }
}
</script>

<style scoped>
@keyframes pulse {
  0% {
    opacity: 1;
  }
  50% {
    opacity: 0.5;
  }
  100% {
    opacity: 1;
  }
}

.a-statistic-card {
  background: #fff;
  padding: 20px;
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.06);
}

.a-statistic-card h3 {
  margin: 0 0 8px 0;
  font-size: 14px;
  color: #666;
}

.a-statistic-card .value {
  font-size: 28px;
  font-weight: 600;
  margin: 0 0 4px 0;
}

.a-statistic-card .description {
  color: #999;
  font-size: 12px;
}

:deep(.ant-table) {
  background: #fff;
}

:deep(.ant-table-thead > tr > th) {
  background: #fafafa;
  font-weight: 600;
}

:deep(.ant-table-tbody > tr:hover > td) {
  background: #f6f8fa;
}
</style>
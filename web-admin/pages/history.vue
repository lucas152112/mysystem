<template>
  <div>
    <!-- 頁面標題 -->
    <div style="margin-bottom: 24px;">
      <h1 style="font-size: 24px; font-weight: 600; margin-bottom: 8px;">執行記錄</h1>
      <p style="color: #666;">查看所有功能的執行歷史記錄和結果</p>
    </div>
    
    <!-- 篩選欄 -->
    <a-card style="margin-bottom: 24px;">
      <a-form layout="inline" :model="filterForm">
        <a-form-item label="功能">
          <a-select
            v-model:value="filterForm.function_id"
            placeholder="選擇功能"
            style="width: 180px"
            allow-clear
          >
            <a-select-option 
              v-for="func in functions" 
              :key="func.id" 
              :value="func.id"
            >
              {{ func.name }}
            </a-select-option>
          </a-select>
        </a-form-item>
        
        <a-form-item label="結果">
          <a-select
            v-model:value="filterForm.result"
            placeholder="選擇結果"
            style="width: 120px"
            allow-clear
          >
            <a-select-option value="success">成功</a-select-option>
            <a-select-option value="failed">失敗</a-select-option>
            <a-select-option value="timeout">超時</a-select-option>
            <a-select-option value="stopped">停止</a-select-option>
          </a-select>
        </a-form-item>
        
        <a-form-item label="時間範圍">
          <a-range-picker
            v-model:value="filterForm.dateRange"
            style="width: 240px"
            :placeholder="['開始日期', '結束日期']"
          />
        </a-form-item>
        
        <a-form-item>
          <a-space :size="16">
            <a-button type="primary" @click="handleSearch">
              搜尋
            </a-button>
            <a-button @click="resetFilters">
              重置
            </a-button>
          </a-space>
        </a-form-item>
      </a-form>
    </a-card>
    
    <!-- 執行記錄表格 -->
    <a-card>
      <template #extra>
        <a-space :size="8">
          <a-button 
            size="small"
            @click="refreshExecutions"
            :loading="loading"
          >
            刷新
          </a-button>
          <a-button 
            size="small"
            @click="exportExecutions"
          >
            導出
          </a-button>
          <a-button 
            size="small"
            danger
            @click="clearOldExecutions"
            :disabled="executions.length === 0"
          >
            清理舊記錄
          </a-button>
        </a-space>
      </template>
      
      <a-table
        :dataSource="filteredExecutions"
        :columns="columns"
        :loading="loading"
        :pagination="pagination"
        rowKey="id"
        @change="handleTableChange"
      >
        <!-- 功能名稱列 -->
        <template #function_name="{ record }">
          <span style="font-weight: 500; color: #1890ff;">
            {{ getFunctionName(record.function_id) }}
          </span>
        </template>
        
        <!-- 結果列 -->
        <template #result="{ record }">
          <a-tag :color="getResultColor(record.result)">
            {{ getResultText(record.result) }}
          </a-tag>
        </template>
        
        <!-- 執行時間列 -->
        <template #duration="{ record }">
          <span v-if="record.end_time">
            {{ calculateDuration(record.start_time, record.end_time) }}
          </span>
          <span v-else style="color: #999;">執行中</span>
        </template>
        
        <!-- 輸出預覽列 -->
        <template #output_preview="{ record }">
          <div style="max-width: 200px;">
            <a-tooltip :title="record.output || record.error || '無輸出'">
              <span style="font-family: monospace; font-size: 12px;">
                {{ truncateText(record.output || record.error || '無輸出', 30) }}
              </span>
            </a-tooltip>
          </div>
        </template>
        
        <!-- 操作列 -->
        <template #action="{ record }">
          <a-space :size="8">
            <a-button
              size="small"
              type="link"
              @click="viewDetails(record)"
            >
              詳情
            </a-button>
            <a-button
              size="small"
              type="link"
              @click="reExecute(record)"
            >
              重新執行
            </a-button>
          </a-space>
        </template>
      </a-table>
    </a-card>
    
    <!-- 執行詳情模態框 -->
    <a-modal
      v-model:visible="showDetailModal"
      :title="detailTitle"
      width="800px"
      :footer="null"
    >
      <div v-if="selectedExecution">
        <!-- 基本資訊 -->
        <a-descriptions 
          :column="2" 
          bordered 
          size="small"
          style="margin-bottom: 16px;"
        >
          <a-descriptions-item label="功能">
            {{ getFunctionName(selectedExecution.function_id) }}
          </a-descriptions-item>
          <a-descriptions-item label="執行結果">
            <a-tag :color="getResultColor(selectedExecution.result)">
              {{ getResultText(selectedExecution.result) }}
            </a-tag>
          </a-descriptions-item>
          <a-descriptions-item label="開始時間">
            {{ formatDateTime(selectedExecution.start_time) }}
          </a-descriptions-item>
          <a-descriptions-item label="結束時間">
            {{ selectedExecution.end_time ? formatDateTime(selectedExecution.end_time) : '執行中' }}
          </a-descriptions-item>
          <a-descriptions-item label="執行時長">
            {{ selectedExecution.end_time ? calculateDuration(selectedExecution.start_time, selectedExecution.end_time) : '執行中' }}
          </a-descriptions-item>
          <a-descriptions-item label="退出代碼">
            {{ selectedExecution.exit_code !== null ? selectedExecution.exit_code : '-' }}
          </a-descriptions-item>
        </a-descriptions>
        
        <!-- 標準輸出 -->
        <a-card title="標準輸出" size="small" style="margin-bottom: 16px;">
          <pre style="margin: 0; font-family: 'SFMono-Regular', Consolas, 'Liberation Mono', Menlo, monospace; font-size: 12px; background: #f6f8fa; padding: 12px; border-radius: 4px; max-height: 200px; overflow: auto;">
{{ selectedExecution.output || '無輸出' }}
          </pre>
        </a-card>
        
        <!-- 錯誤輸出 -->
        <a-card v-if="selectedExecution.error" title="錯誤輸出" size="small">
          <pre style="margin: 0; font-family: 'SFMono-Regular', Consolas, 'Liberation Mono', Menlo, monospace; font-size: 12px; background: #fff2f0; padding: 12px; border-radius: 4px; max-height: 200px; overflow: auto; color: #ff4d4f;">
{{ selectedExecution.error }}
          </pre>
        </a-card>
      </div>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { message, type TableProps } from 'ant-design-vue'
import { useFunctionStore } from '~/stores/functions'
import type { Dayjs } from 'dayjs'

interface Execution {
  id: string
  function_id: string
  start_time: string
  end_time: string | null
  exit_code: number | null
  output: string | null
  error: string | null
  result: 'success' | 'failed' | 'timeout' | 'stopped'
}

const functionStore = useFunctionStore()

// 狀態
const executions = ref<Execution[]>([])
const functions = ref<any[]>([])
const loading = ref(false)

// 篩選表單
const filterForm = ref({
  function_id: undefined as string | undefined,
  result: undefined as string | undefined,
  dateRange: undefined as [Dayjs, Dayjs] | undefined,
})

// 分頁
const pagination = ref({
  current: 1,
  pageSize: 10,
  total: 0,
  showSizeChanger: true,
  showQuickJumper: true,
  showTotal: (total: number) => `共 ${total} 條記錄`,
})

// 模態框
const showDetailModal = ref(false)
const selectedExecution = ref<Execution | null>(null)

// 表格列定義
const columns = [
  {
    title: '執行ID',
    dataIndex: 'id',
    key: 'id',
    width: 120,
    ellipsis: true,
  },
  {
    title: '功能',
    dataIndex: 'function_id',
    key: 'function_name',
    slots: { customRender: 'function_name' },
    width: 150,
  },
  {
    title: '開始時間',
    dataIndex: 'start_time',
    key: 'start_time',
    width: 120,
    customRender: ({ text }: { text: string }) => formatDateTime(text),
  },
  {
    title: '結果',
    dataIndex: 'result',
    key: 'result',
    slots: { customRender: 'result' },
    width: 100,
  },
  {
    title: '執行時長',
    dataIndex: 'end_time',
    key: 'duration',
    slots: { customRender: 'duration' },
    width: 100,
  },
  {
    title: '輸出預覽',
    key: 'output_preview',
    slots: { customRender: 'output_preview' },
    ellipsis: true,
  },
  {
    title: '操作',
    key: 'action',
    slots: { customRender: 'action' },
    width: 120,
  },
]

// 計算屬性
const filteredExecutions = computed(() => {
  let filtered = executions.value
  
  // 按功能篩選
  if (filterForm.value.function_id) {
    filtered = filtered.filter(e => e.function_id === filterForm.value.function_id)
  }
  
  // 按結果篩選
  if (filterForm.value.result) {
    filtered = filtered.filter(e => e.result === filterForm.value.result)
  }
  
  // 按時間範圍篩選
  if (filterForm.value.dateRange && filterForm.value.dateRange.length === 2) {
    const [start, end] = filterForm.value.dateRange
    const startTime = start.startOf('day').toDate()
    const endTime = end.endOf('day').toDate()
    
    filtered = filtered.filter(e => {
      const execTime = new Date(e.start_time)
      return execTime >= startTime && execTime <= endTime
    })
  }
  
  // 更新分頁總數
  pagination.value.total = filtered.length
  
  // 分頁
  const start = (pagination.value.current - 1) * pagination.value.pageSize
  const end = start + pagination.value.pageSize
  return filtered.slice(start, end)
})

// 詳情標題
const detailTitle = computed(() => {
  if (!selectedExecution.value) return '執行詳情'
  const funcName = getFunctionName(selectedExecution.value.function_id)
  return `執行詳情 - ${funcName}`
})

// 初始化
onMounted(async () => {
  await loadFunctions()
  await loadExecutions()
})

// 加載功能列表
const loadFunctions = async () => {
  try {
    const data = await functionStore.fetchFunctions()
    functions.value = data
  } catch (error) {
    message.error('加載功能列表失敗')
  }
}

// 加載執行記錄
const loadExecutions = async () => {
  loading.value = true
  try {
    // 這裡應該實現獲取所有執行記錄的API
    // 目前先模擬數據
    executions.value = await mockFetchExecutions()
  } catch (error) {
    message.error('加載執行記錄失敗')
  } finally {
    loading.value = false
  }
}

// 模擬獲取執行記錄
const mockFetchExecutions = async (): Promise<Execution[]> => {
  // 模擬數據
  return []
}

// 處理搜尋
const handleSearch = () => {
  pagination.value.current = 1
}

// 重置篩選
const resetFilters = () => {
  filterForm.value = {
    function_id: undefined,
    result: undefined,
    dateRange: undefined,
  }
  pagination.value.current = 1
}

// 處理表格變化
const handleTableChange: TableProps['onChange'] = (pag) => {
  if (pag.current !== undefined) {
    pagination.value.current = pag.current
  }
  if (pag.pageSize !== undefined) {
    pagination.value.pageSize = pag.pageSize
  }
}

// 刷新執行記錄
const refreshExecutions = async () => {
  await loadExecutions()
  message.success('刷新成功')
}

// 導出執行記錄
const exportExecutions = () => {
  const data = {
    timestamp: new Date().toISOString(),
    count: executions.value.length,
    executions: executions.value,
  }
  
  const json = JSON.stringify(data, null, 2)
  const blob = new Blob([json], { type: 'application/json' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = `executions-export-${new Date().toISOString().slice(0, 10)}.json`
  document.body.appendChild(a)
  a.click()
  document.body.removeChild(a)
  URL.revokeObjectURL(url)
  
  message.success('執行記錄導出成功')
}

// 清理舊記錄
const clearOldExecutions = () => {
  message.info('清理舊記錄功能開發中')
}

// 查看詳情
const viewDetails = (execution: Execution) => {
  selectedExecution.value = execution
  showDetailModal.value = true
}

// 重新執行
const reExecute = async (execution: Execution) => {
  try {
    await functionStore.executeFunction(execution.function_id, {})
    message.success('功能重新執行中...')
    
    // 延遲刷新
    setTimeout(() => {
      loadExecutions()
    }, 1000)
  } catch (error) {
    message.error('重新執行失敗')
  }
}

// 獲取功能名稱
const getFunctionName = (functionId: string) => {
  const func = functions.value.find(f => f.id === functionId)
  return func ? func.name : functionId
}

// 計算執行時長
const calculateDuration = (start: string, end: string) => {
  const startTime = new Date(start).getTime()
  const endTime = new Date(end).getTime()
  const durationMs = endTime - startTime
  
  if (durationMs < 1000) {
    return `${durationMs}ms`
  } else if (durationMs < 60000) {
    return `${(durationMs / 1000).toFixed(2)}s`
  } else {
    const minutes = Math.floor(durationMs / 60000)
    const seconds = ((durationMs % 60000) / 1000).toFixed(2)
    return `${minutes}m ${seconds}s`
  }
}

// 格式化日期時間
const formatDateTime = (timestamp: string) => {
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

// 截斷文本
const truncateText = (text: string, maxLength: number) => {
  if (text.length <= maxLength) return text
  return text.substring(0, maxLength) + '...'
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

pre {
  white-space: pre-wrap;
  word-wrap: break-word;
}
</style>
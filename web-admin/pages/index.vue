<template>
  <div>
    <!-- 儀表板標題 -->
    <div style="margin-bottom: 24px;">
      <h1 style="font-size: 24px; font-weight: 600; margin-bottom: 8px;">系統儀表板</h1>
      <p style="color: #666;">監控和管理全部系統功能的一鍵操作界面</p>
    </div>
    
    <!-- 狀態統計卡片 -->
    <a-row :gutter="[16, 16]" style="margin-bottom: 24px;">
      <a-col :span="6">
        <a-statistic title="總功能數" :value="stats.totalFunctions" />
      </a-col>
      <a-col :span="6">
        <a-statistic title="運行中" :value="stats.running" :value-style="{ color: '#52c41a' }" />
      </a-col>
      <a-col :span="6">
        <a-statistic title="已停止" :value="stats.stopped" :value-style="{ color: '#faad14' }" />
      </a-col>
      <a-col :span="6">
        <a-statistic title="錯誤" :value="stats.error" :value-style="{ color: '#ff4d4f' }" />
      </a-col>
    </a-row>
    
    <!-- 功能按鈕網格 -->
    <div style="margin-bottom: 24px;">
      <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 16px;">
        <h2 style="font-size: 18px; font-weight: 500;">功能按鈕</h2>
        <a-button type="primary" @click="showAddFunctionModal = true">
          新增功能
        </a-button>
      </div>
      
      <a-row :gutter="[16, 16]">
        <a-col :span="6" v-for="functionItem in functions" :key="functionItem.id">
          <FunctionCard :function="functionItem" @execute="handleExecute" />
        </a-col>
        
        <!-- 空狀態 -->
        <a-col v-if="functions.length === 0" :span="24">
          <a-empty description="暫無功能配置">
            <template #extra>
              <a-button type="primary" @click="showAddFunctionModal = true">新增第一個功能</a-button>
            </template>
          </a-empty>
        </a-col>
      </a-row>
    </div>
    
    <!-- 實時狀態圖表 -->
    <a-card title="狀態監控" style="margin-bottom: 24px;">
      <a-row :gutter="[16, 16]">
        <a-col :span="12">
          <div style="height: 300px;">
            <!-- 這裡可以放置狀態圖表 -->
            <div style="display: flex; align-items: center; justify-content: center; height: 100%;">
              <p>狀態分佈圖表（開發中）</p>
            </div>
          </div>
        </a-col>
        <a-col :span="12">
          <div style="height: 300px;">
            <!-- 這裡可以放置執行歷史圖表 -->
            <div style="display: flex; align-items: center; justify-content: center; height: 100%;">
              <p>執行歷史圖表（開發中）</p>
            </div>
          </div>
        </a-col>
      </a-row>
    </a-card>
    
    <!-- 快速操作 -->
    <a-card title="快速操作">
      <a-space :size="16">
        <a-button type="primary" @click="refreshAllStatus">刷新全部狀態</a-button>
        <a-button @click="stopAllFunctions">停止全部功能</a-button>
        <a-button @click="exportFunctions">導出功能配置</a-button>
        <a-button @click="importFunctions">導入功能配置</a-button>
      </a-space>
    </a-card>
    
    <!-- 新增功能模態框 -->
    <AddFunctionModal
      v-model:visible="showAddFunctionModal"
      @success="handleAddFunctionSuccess"
    />
    
    <!-- 執行確認模態框 -->
    <a-modal
      v-model:visible="showExecuteModal"
      title="執行確認"
      @ok="confirmExecute"
      @cancel="cancelExecute"
    >
      <p>確定要執行「{{ selectedFunction?.name }}」功能嗎？</p>
      <a-form layout="vertical" :model="executeForm">
        <a-form-item label="執行參數（可選）">
          <a-textarea
            v-model:value="executeForm.args"
            placeholder="每行一個參數"
            :rows="3"
          />
        </a-form-item>
        <a-form-item label="超時時間（秒）">
          <a-input-number
            v-model:value="executeForm.timeout"
            :min="1"
            :max="300"
            style="width: 100%"
          />
        </a-form-item>
      </a-form>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import FunctionCard from '~/components/FunctionCard.vue'
import AddFunctionModal from '~/components/AddFunctionModal.vue'
import { useFunctionStore } from '~/stores/functions'
import { useStatusStore } from '~/stores/status'
import { message } from 'ant-design-vue'

const functionStore = useFunctionStore()
const statusStore = useStatusStore()

const stats = ref({
  totalFunctions: 0,
  running: 0,
  stopped: 0,
  error: 0,
})

const functions = ref<any[]>([])
const showAddFunctionModal = ref(false)
const showExecuteModal = ref(false)
const selectedFunction = ref<any>(null)
const executeForm = ref({
  args: '',
  timeout: 30,
})

// 初始化數據
onMounted(async () => {
  await loadFunctions()
  updateStats()
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

// 更新統計數據
const updateStats = () => {
  const allFunctions = functions.value
  stats.value.totalFunctions = allFunctions.length
  stats.value.running = allFunctions.filter(f => f.status === 'running').length
  stats.value.stopped = allFunctions.filter(f => f.status === 'stopped').length
  stats.value.error = allFunctions.filter(f => f.status === 'error').length
}

// 處理執行功能
const handleExecute = (functionItem: any) => {
  selectedFunction.value = functionItem
  executeForm.value.args = functionItem.args?.join('\n') || ''
  executeForm.value.timeout = functionItem.timeout || 30
  showExecuteModal.value = true
}

// 確認執行
const confirmExecute = async () => {
  if (!selectedFunction.value) return
  
  try {
    const args = executeForm.value.args
      .split('\n')
      .map(arg => arg.trim())
      .filter(arg => arg.length > 0)
    
    await functionStore.executeFunction(selectedFunction.value.id, {
      args,
      timeout: executeForm.value.timeout,
    })
    
    message.success('功能執行中...')
    showExecuteModal.value = false
    
    // 刷新狀態
    setTimeout(() => {
      loadFunctions()
      statusStore.refreshStatus()
    }, 1000)
  } catch (error) {
    message.error('執行功能失敗')
  }
}

// 取消執行
const cancelExecute = () => {
  showExecuteModal.value = false
  selectedFunction.value = null
}

// 處理新增功能成功
const handleAddFunctionSuccess = () => {
  showAddFunctionModal.value = false
  loadFunctions()
  updateStats()
}

// 刷新全部狀態
const refreshAllStatus = async () => {
  try {
    await statusStore.refreshStatus()
    await loadFunctions()
    updateStats()
    message.success('狀態刷新成功')
  } catch (error) {
    message.error('狀態刷新失敗')
  }
}

// 停止全部功能
const stopAllFunctions = async () => {
  // 實現停止全部功能的邏輯
  message.info('停止全部功能功能開發中')
}

// 導出功能配置
const exportFunctions = () => {
  // 實現導出功能
  message.info('導出功能配置開發中')
}

// 導入功能配置
const importFunctions = () => {
  // 實現導入功能
  message.info('導入功能配置開發中')
}
</script>
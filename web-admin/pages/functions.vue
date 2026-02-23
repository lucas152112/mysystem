<template>
  <div>
    <!-- 頁面標題 -->
    <div style="margin-bottom: 24px;">
      <h1 style="font-size: 24px; font-weight: 600; margin-bottom: 8px;">功能管理</h1>
      <p style="color: #666;">管理系統功能，配置執行命令和參數</p>
    </div>
    
    <!-- 操作欄 -->
    <div style="margin-bottom: 24px; display: flex; justify-content: space-between; align-items: center;">
      <div style="display: flex; gap: 16px; align-items: center;">
        <a-input-search
          v-model:value="searchKeyword"
          placeholder="搜尋功能名稱或描述"
          style="width: 300px"
          @search="handleSearch"
          @change="handleSearch"
        />
        
        <a-select
          v-model:value="statusFilter"
          style="width: 120px"
          placeholder="狀態篩選"
          @change="handleFilter"
        >
          <a-select-option value="">全部狀態</a-select-option>
          <a-select-option value="running">運行中</a-select-option>
          <a-select-option value="stopped">已停止</a-select-option>
          <a-select-option value="error">錯誤</a-select-option>
        </a-select>
      </div>
      
      <a-space :size="16">
        <a-button @click="refreshFunctions">
          <template #icon><RedoOutlined /></template>
          刷新
        </a-button>
        <a-button type="primary" @click="showAddModal">
          <template #icon><PlusOutlined /></template>
          新增功能
        </a-button>
      </a-space>
    </div>
    
    <!-- 功能表格 -->
    <a-table
      :dataSource="filteredFunctions"
      :columns="columns"
      :loading="functionStore.loading"
      :pagination="pagination"
      rowKey="id"
      @change="handleTableChange"
    >
      <!-- 狀態列 -->
      <template #status="{ record }">
        <a-tag :color="getStatusColor(record.status)">
          {{ getStatusText(record.status) }}
        </a-tag>
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
            type="primary"
            @click="handleExecute(record)"
            :disabled="record.status === 'running'"
          >
            {{ record.status === 'running' ? '執行中' : '執行' }}
          </a-button>
          <a-button
            size="small"
            @click="handleEdit(record)"
          >
            編輯
          </a-button>
          <a-button
            size="small"
            danger
            @click="handleDelete(record)"
          >
            刪除
          </a-button>
        </a-space>
      </template>
    </a-table>
    
    <!-- 新增功能模態框 -->
    <AddFunctionModal
      v-model:visible="showAddFunctionModal"
      @success="handleAddSuccess"
    />
    
    <!-- 編輯功能模態框 -->
    <EditFunctionModal
      v-model:visible="showEditFunctionModal"
      :function="editingFunction"
      @success="handleEditSuccess"
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
    
    <!-- 刪除確認模態框 -->
    <a-modal
      v-model:visible="showDeleteModal"
      title="刪除確認"
      @ok="confirmDelete"
      @cancel="cancelDelete"
    >
      <p>確定要刪除「{{ deletingFunction?.name }}」功能嗎？</p>
      <p style="color: #ff4d4f;">此操作不可恢復，請謹慎操作！</p>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { message, type TableProps } from 'ant-design-vue'
import { RedoOutlined, PlusOutlined } from '@ant-design/icons-vue'
import { useFunctionStore } from '~/stores/functions'
import AddFunctionModal from '~/components/AddFunctionModal.vue'
import EditFunctionModal from '~/components/EditFunctionModal.vue'

const functionStore = useFunctionStore()

// 搜尋和篩選
const searchKeyword = ref('')
const statusFilter = ref('')

// 模態框狀態
const showAddFunctionModal = ref(false)
const showEditFunctionModal = ref(false)
const showExecuteModal = ref(false)
const showDeleteModal = ref(false)

// 當前操作的功能
const editingFunction = ref<any>(null)
const selectedFunction = ref<any>(null)
const deletingFunction = ref<any>(null)

// 執行表單
const executeForm = ref({
  args: '',
  timeout: 30,
})

// 分頁配置
const pagination = ref({
  current: 1,
  pageSize: 10,
  total: 0,
  showSizeChanger: true,
  showQuickJumper: true,
  showTotal: (total: number) => `共 ${total} 個功能`,
})

// 表格列定義
const columns = [
  {
    title: '功能名稱',
    dataIndex: 'name',
    key: 'name',
    width: 150,
  },
  {
    title: '描述',
    dataIndex: 'description',
    key: 'description',
    ellipsis: true,
  },
  {
    title: '命令',
    dataIndex: 'command',
    key: 'command',
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
    title: '最後執行',
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
    title: '創建時間',
    dataIndex: 'created_at',
    key: 'created_at',
    width: 120,
    customRender: ({ text }: { text: string }) => formatTime(text),
  },
  {
    title: '操作',
    key: 'action',
    slots: { customRender: 'action' },
    width: 200,
  },
]

// 過濾後的功能列表
const filteredFunctions = computed(() => {
  let functions = functionStore.functions
  
  // 搜尋過濾
  if (searchKeyword.value) {
    const keyword = searchKeyword.value.toLowerCase()
    functions = functions.filter(f => 
      f.name.toLowerCase().includes(keyword) ||
      (f.description?.toLowerCase() || '').includes(keyword) ||
      f.command.toLowerCase().includes(keyword)
    )
  }
  
  // 狀態過濾
  if (statusFilter.value) {
    functions = functions.filter(f => f.status === statusFilter.value)
  }
  
  // 更新分頁總數
  pagination.value.total = functions.length
  
  // 分頁
  const start = (pagination.value.current - 1) * pagination.value.pageSize
  const end = start + pagination.value.pageSize
  return functions.slice(start, end)
})

// 初始化數據
onMounted(async () => {
  await loadFunctions()
})

// 加載功能列表
const loadFunctions = async () => {
  try {
    await functionStore.fetchFunctions()
  } catch (error) {
    message.error('加載功能列表失敗')
  }
}

// 刷新功能列表
const refreshFunctions = async () => {
  await loadFunctions()
  message.success('刷新成功')
}

// 處理搜尋
const handleSearch = () => {
  pagination.value.current = 1
}

// 處理篩選
const handleFilter = () => {
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

// 顯示新增模態框
const showAddModal = () => {
  showAddFunctionModal.value = true
}

// 處理新增成功
const handleAddSuccess = () => {
  showAddFunctionModal.value = false
  loadFunctions()
  message.success('功能新增成功')
}

// 處理編輯
const handleEdit = (func: any) => {
  editingFunction.value = func
  showEditFunctionModal.value = true
}

// 處理編輯成功
const handleEditSuccess = () => {
  showEditFunctionModal.value = false
  loadFunctions()
  message.success('功能更新成功')
}

// 處理執行
const handleExecute = (func: any) => {
  selectedFunction.value = func
  executeForm.value.args = func.args?.join('\n') || ''
  executeForm.value.timeout = func.timeout || 30
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
    selectedFunction.value = null
    
    // 刷新狀態
    setTimeout(() => {
      loadFunctions()
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

// 處理刪除
const handleDelete = (func: any) => {
  deletingFunction.value = func
  showDeleteModal.value = true
}

// 確認刪除
const confirmDelete = async () => {
  if (!deletingFunction.value) return
  
  try {
    await functionStore.deleteFunction(deletingFunction.value.id)
    message.success('功能刪除成功')
    showDeleteModal.value = false
    deletingFunction.value = null
    
    // 如果刪除的是最後一頁的最後一項，調整分頁
    if (filteredFunctions.value.length === 0 && pagination.value.current > 1) {
      pagination.value.current--
    }
  } catch (error) {
    message.error('刪除功能失敗')
  }
}

// 取消刪除
const cancelDelete = () => {
  showDeleteModal.value = false
  deletingFunction.value = null
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

// 狀態顏色映射
const getStatusColor = (status: string) => {
  switch (status) {
    case 'running': return 'success'
    case 'stopped': return 'warning'
    case 'error': return 'error'
    default: return 'default'
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
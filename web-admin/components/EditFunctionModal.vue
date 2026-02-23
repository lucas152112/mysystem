<template>
  <a-modal
    v-model:visible="visible"
    title="編輯功能"
    width="600px"
    :confirm-loading="loading"
    @ok="handleSubmit"
    @cancel="handleCancel"
  >
    <a-form
      ref="formRef"
      :model="formState"
      :rules="rules"
      layout="vertical"
      @finish="handleSubmit"
    >
      <a-form-item label="功能名稱" name="name">
        <a-input
          v-model:value="formState.name"
          placeholder="例如：重啟服務器、備份數據庫"
        />
      </a-form-item>
      
      <a-form-item label="功能描述" name="description">
        <a-textarea
          v-model:value="formState.description"
          placeholder="描述此功能的用途和注意事項"
          :rows="3"
        />
      </a-form-item>
      
      <a-form-item label="執行命令" name="command">
        <a-input
          v-model:value="formState.command"
          placeholder="例如：systemctl restart nginx"
        />
        <template #extra>
          <span style="font-size: 12px; color: #999;">
            支持系統命令或腳本路徑（如：bash /path/to/script.sh）
          </span>
        </template>
      </a-form-item>
      
      <a-form-item label="命令參數" name="args">
        <a-textarea
          v-model:value="formState.args"
          placeholder="每行一個參數"
          :rows="3"
        />
        <template #extra>
          <span style="font-size: 12px; color: #999;">
            每行一個參數，可選。例如：<br>--verbose<br>--output=log.txt
          </span>
        </template>
      </a-form-item>
      
      <a-row :gutter="16">
        <a-col :span="12">
          <a-form-item label="超時時間（秒）" name="timeout">
            <a-input-number
              v-model:value="formState.timeout"
              :min="1"
              :max="300"
              style="width: 100%"
            />
          </a-form-item>
        </a-col>
        <a-col :span="12">
          <a-form-item label="當前狀態" name="status">
            <a-select
              v-model:value="formState.status"
              style="width: 100%"
              :disabled="true"
            >
              <a-select-option value="stopped">已停止</a-select-option>
              <a-select-option value="running">運行中</a-select-option>
              <a-select-option value="error">錯誤</a-select-option>
            </a-select>
          </a-form-item>
        </a-col>
      </a-row>
      
      <!-- 執行記錄 -->
      <a-form-item label="最後執行時間">
        <span style="color: #666;">
          {{ lastExecutedText }}
        </span>
      </a-form-item>
      
      <a-form-item label="最後執行結果">
        <a-tag :color="getResultColor(formState.last_status)">
          {{ getResultText(formState.last_status) }}
        </a-tag>
      </a-form-item>
      
      <!-- 命令預覽 -->
      <a-form-item label="命令預覽">
        <div style="background: #f6f8fa; padding: 12px; border-radius: 4px; font-family: monospace;">
          <code>{{ commandPreview }}</code>
        </div>
      </a-form-item>
    </a-form>
  </a-modal>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { message, type FormInstance } from 'ant-design-vue'
import { useFunctionStore } from '~/stores/functions'

interface Props {
  visible: boolean
  function: any | null
}

interface Emits {
  (event: 'update:visible', value: boolean): void
  (event: 'success'): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const formRef = ref<FormInstance>()
const loading = ref(false)
const functionStore = useFunctionStore()

// 表單狀態
const formState = ref({
  id: '',
  name: '',
  description: '',
  command: '',
  args: '',
  timeout: 30,
  status: 'stopped' as 'running' | 'stopped' | 'error',
  last_status: null as 'success' | 'failed' | 'timeout' | 'stopped' | null,
  last_executed: null as string | null,
})

// 表單驗證規則
const rules = {
  name: [
    { required: true, message: '請輸入功能名稱', trigger: 'blur' },
    { max: 100, message: '名稱不能超過100個字符', trigger: 'blur' },
  ],
  command: [
    { required: true, message: '請輸入執行命令', trigger: 'blur' },
  ],
  timeout: [
    { required: true, message: '請設置超時時間', trigger: 'blur' },
  ],
}

// 最後執行時間文本
const lastExecutedText = computed(() => {
  if (!formState.value.last_executed) {
    return '從未執行'
  }
  
  try {
    const date = new Date(formState.value.last_executed)
    return date.toLocaleString('zh-TW', {
      year: 'numeric',
      month: 'short',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
      second: '2-digit',
    })
  } catch (error) {
    return '時間格式錯誤'
  }
})

// 命令預覽
const commandPreview = computed(() => {
  const command = formState.value.command
  const args = formState.value.args
    .split('\n')
    .map(arg => arg.trim())
    .filter(arg => arg.length > 0)
    .join(' ')
  
  return args ? `${command} ${args}` : command
})

// 監視傳入的function
watch(() => props.function, (newFunc) => {
  if (newFunc) {
    formState.value = {
      id: newFunc.id,
      name: newFunc.name,
      description: newFunc.description || '',
      command: newFunc.command,
      args: newFunc.args?.join('\n') || '',
      timeout: newFunc.timeout || 30,
      status: newFunc.status,
      last_status: newFunc.last_status || null,
      last_executed: newFunc.last_executed || null,
    }
  }
}, { immediate: true })

// 處理提交
const handleSubmit = async () => {
  if (!formRef.value) return
  
  try {
    // 驗證表單
    await formRef.value.validate()
    
    loading.value = true
    
    // 準備數據
    const args = formState.value.args
      .split('\n')
      .map(arg => arg.trim())
      .filter(arg => arg.length > 0)
    
    const requestData = {
      name: formState.value.name,
      description: formState.value.description || undefined,
      command: formState.value.command,
      args: args.length > 0 ? args : undefined,
      timeout: formState.value.timeout,
    }
    
    // 調用 API
    await functionStore.updateFunction(formState.value.id, requestData)
    
    // 成功處理
    message.success('功能更新成功')
    emit('success')
    handleCancel()
    
  } catch (error) {
    // 錯誤已在 store 中處理
    console.error('更新功能失敗:', error)
  } finally {
    loading.value = false
  }
}

// 處理取消
const handleCancel = () => {
  // 重置表單
  if (formRef.value) {
    formRef.value.resetFields()
  }
  
  formState.value = {
    id: '',
    name: '',
    description: '',
    command: '',
    args: '',
    timeout: 30,
    status: 'stopped',
    last_status: null,
    last_executed: null,
  }
  
  emit('update:visible', false)
}

// 結果顏色映射
const getResultColor = (result: string | null) => {
  if (!result) return 'default'
  
  switch (result) {
    case 'success': return 'success'
    case 'failed': return 'error'
    case 'timeout': return 'warning'
    case 'stopped': return 'default'
    default: return 'default'
  }
}

// 結果文本
const getResultText = (result: string | null) => {
  if (!result) return '-'
  
  switch (result) {
    case 'success': return '成功'
    case 'failed': return '失敗'
    case 'timeout': return '超時'
    case 'stopped': return '停止'
    default: return '未知'
  }
}

// 監聽 visible 屬性變化
watch(() => props.visible, (newVal) => {
  if (!newVal) {
    // 模態框關閉時重置表單
    if (formRef.value) {
      formRef.value.resetFields()
    }
  }
})
</script>

<style scoped>
:deep(.ant-form-item-extra) {
  margin-top: 4px;
  font-size: 12px;
}
</style>
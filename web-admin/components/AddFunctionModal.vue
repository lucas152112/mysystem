<template>
  <a-modal
    v-model:visible="visible"
    title="新增功能"
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
          <a-form-item label="初始狀態" name="initialStatus">
            <a-select
              v-model:value="formState.initialStatus"
              style="width: 100%"
            >
              <a-select-option value="stopped">已停止</a-select-option>
              <a-select-option value="running">運行中</a-select-option>
            </a-select>
          </a-form-item>
        </a-col>
      </a-row>
      
      <!-- 命令預覽 -->
      <a-form-item label="命令預覽">
        <div style="background: #f6f8fa; padding: 12px; border-radius: 4px; font-family: monospace;">
          <code>{{ commandPreview }}</code>
        </div>
      </a-form-item>
      
      <!-- 安全警告 -->
      <a-alert
        message="安全提示"
        description="請謹慎添加系統命令，確保只執行可信的命令。建議使用白名單限制可執行的命令。"
        type="warning"
        show-icon
        style="margin-bottom: 16px;"
      />
    </a-form>
  </a-modal>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { message, type FormInstance } from 'ant-design-vue'
import { useFunctionStore } from '~/stores/functions'

interface Props {
  visible: boolean
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
  name: '',
  description: '',
  command: '',
  args: '',
  timeout: 30,
  initialStatus: 'stopped' as 'stopped' | 'running',
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
    await functionStore.createFunction(requestData)
    
    // 成功處理
    message.success('功能創建成功')
    emit('success')
    handleCancel()
    
  } catch (error) {
    // 錯誤已在 store 中處理
    console.error('創建功能失敗:', error)
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
    name: '',
    description: '',
    command: '',
    args: '',
    timeout: 30,
    initialStatus: 'stopped',
  }
  
  emit('update:visible', false)
}

// 監聽 visible 屬性變化
watch(() => props.visible, (newVal) => {
  if (newVal) {
    // 模態框打開時重置表單
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
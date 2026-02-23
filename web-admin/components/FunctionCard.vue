<template>
  <a-card hoverable :class="['function-card', `status-${functionItem.status}`]">
    <!-- 狀態燈號 -->
    <div class="status-indicator" :style="{ backgroundColor: statusColor }" />
    
    <!-- 功能資訊 -->
    <div class="function-content">
      <div class="function-header">
        <h3 class="function-name">{{ functionItem.name }}</h3>
        <a-tag :color="statusTagColor">{{ statusText }}</a-tag>
      </div>
      
      <p class="function-description" v-if="functionItem.description">
        {{ functionItem.description }}
      </p>
      
      <!-- 最後執行時間 -->
      <div class="function-meta">
        <span v-if="functionItem.last_executed">
          最後執行: {{ formatTime(functionItem.last_executed) }}
        </span>
        <span v-else>從未執行</span>
      </div>
      
      <!-- 命令預覽 -->
      <div class="command-preview">
        <code>{{ functionItem.command }}</code>
        <span v-if="functionItem.args?.length > 0"> {{ functionItem.args.join(' ') }}</span>
      </div>
      
      <!-- 操作按鈕 -->
      <div class="function-actions">
        <a-button
          type="primary"
          size="small"
          @click.stop="$emit('execute', functionItem)"
          :disabled="functionItem.status === 'running'"
        >
          {{ functionItem.status === 'running' ? '執行中' : '執行' }}
        </a-button>
        
        <a-space :size="8">
          <a-button
            size="small"
            @click.stop="$emit('edit', functionItem)"
          >
            編輯
          </a-button>
          <a-button
            size="small"
            danger
            @click.stop="$emit('delete', functionItem)"
          >
            刪除
          </a-button>
        </a-space>
      </div>
    </div>
  </a-card>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { message } from 'ant-design-vue'

const props = defineProps<{
  function: any
}>()

const emit = defineEmits<{
  execute: [function: any]
  edit: [function: any]
  delete: [function: any]
}>()

const functionItem = computed(() => props.function)

// 狀態顏色映射
const statusColor = computed(() => {
  switch (functionItem.value.status) {
    case 'running': return '#52c41a'  // 綠色
    case 'stopped': return '#faad14'  // 黃色
    case 'error': return '#ff4d4f'    // 紅色
    default: return '#d9d9d9'
  }
})

// 狀態標籤顏色
const statusTagColor = computed(() => {
  switch (functionItem.value.status) {
    case 'running': return 'success'
    case 'stopped': return 'warning'
    case 'error': return 'error'
    default: return 'default'
  }
})

// 狀態文本
const statusText = computed(() => {
  switch (functionItem.value.status) {
    case 'running': return '運行中'
    case 'stopped': return '已停止'
    case 'error': return '錯誤'
    default: return '未知'
  }
})

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
    return '時間格式錯誤'
  }
}
</script>

<style scoped>
.function-card {
  position: relative;
  height: 220px;
  display: flex;
  flex-direction: column;
  transition: all 0.3s ease;
}

.function-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.status-indicator {
  position: absolute;
  top: 16px;
  right: 16px;
  width: 12px;
  height: 12px;
  border-radius: 50%;
  border: 2px solid white;
  box-shadow: 0 0 4px rgba(0, 0, 0, 0.1);
}

.function-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  height: 100%;
}

.function-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 8px;
}

.function-name {
  font-size: 16px;
  font-weight: 600;
  margin: 0;
  color: #1890ff;
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.function-description {
  color: #666;
  font-size: 13px;
  line-height: 1.4;
  margin-bottom: 12px;
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
}

.function-meta {
  color: #999;
  font-size: 12px;
  margin-bottom: 12px;
}

.command-preview {
  background: #f6f8fa;
  padding: 8px;
  border-radius: 4px;
  margin-bottom: 12px;
  font-family: 'SFMono-Regular', Consolas, 'Liberation Mono', Menlo, monospace;
  font-size: 12px;
  color: #24292e;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.command-preview code {
  background: transparent;
  padding: 0;
  margin: 0;
  font-size: 12px;
}

.function-actions {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-top: auto;
}
</style>
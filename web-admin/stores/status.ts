import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { message } from 'ant-design-vue'
import api from '~/utils/api'

interface StatusLight {
  function_id: string
  status: 'running' | 'stopped' | 'error'
  color: string
  last_executed?: string
  last_status?: 'success' | 'failed' | 'timeout' | 'stopped'
}

export const useStatusStore = defineStore('status', () => {
  const statusLights = ref<StatusLight[]>([])
  const websocketConnected = ref(false)
  const websocketError = ref<string | null>(null)
  const loading = ref(false)
  
  // WebSocket 連接
  let websocket: WebSocket | null = null
  const reconnectInterval = ref<NodeJS.Timeout | null>(null)

  // 獲取所有狀態燈號
  const fetchStatusLights = async (): Promise<StatusLight[]> => {
    loading.value = true
    
    try {
      const response = await api.get('/status/lights')
      statusLights.value = response.data
      return response.data
    } catch (error: any) {
      message.error('獲取狀態燈號失敗: ' + (error.message || '未知錯誤'))
      throw error
    } finally {
      loading.value = false
    }
  }

  // 刷新狀態
  const refreshStatus = async (): Promise<void> => {
    await fetchStatusLights()
  }

  // 連接 WebSocket
  const connectWebSocket = () => {
    const config = useRuntimeConfig()
    const websocketUrl = config.public.websocketUrl
    
    if (!websocketUrl) {
      console.error('WebSocket URL 未配置')
      return
    }

    // 關閉現有連接
    disconnectWebSocket()

    try {
      websocket = new WebSocket(websocketUrl)
      
      websocket.onopen = () => {
        websocketConnected.value = true
        websocketError.value = null
        console.log('WebSocket 連接成功')
      }
      
      websocket.onmessage = (event) => {
        try {
          const data = JSON.parse(event.data)
          handleWebSocketMessage(data)
        } catch (error) {
          console.error('解析 WebSocket 消息失敗:', error)
        }
      }
      
      websocket.onerror = (error) => {
        websocketError.value = 'WebSocket 連接錯誤'
        websocketConnected.value = false
        console.error('WebSocket 錯誤:', error)
      }
      
      websocket.onclose = () => {
        websocketConnected.value = false
        console.log('WebSocket 連接關閉')
        
        // 嘗試重連
        if (!reconnectInterval.value) {
          reconnectInterval.value = setInterval(() => {
            console.log('嘗試重新連接 WebSocket...')
            connectWebSocket()
          }, 5000)
        }
      }
    } catch (error: any) {
      websocketError.value = error.message || '創建 WebSocket 連接失敗'
      message.error('WebSocket 連接失敗: ' + websocketError.value)
    }
  }

  // 處理 WebSocket 消息
  const handleWebSocketMessage = (data: any) => {
    if (data.type === 'status_update') {
      const statusLight = data.data as StatusLight
      
      // 更新對應的功能狀態
      const index = statusLights.value.findIndex(
        light => light.function_id === statusLight.function_id
      )
      
      if (index !== -1) {
        statusLights.value[index] = statusLight
      } else {
        statusLights.value.push(statusLight)
      }
      
      // 發出通知
      if (statusLight.status === 'error') {
        message.warning(`功能狀態異常: ${statusLight.function_id}`)
      }
    }
  }

  // 斷開 WebSocket 連接
  const disconnectWebSocket = () => {
    if (websocket) {
      websocket.close()
      websocket = null
    }
    
    if (reconnectInterval.value) {
      clearInterval(reconnectInterval.value)
      reconnectInterval.value = null
    }
    
    websocketConnected.value = false
  }

  // 根據狀態過濾燈號
  const getStatusLightsByColor = (color: string) => {
    return statusLights.value.filter(light => light.color === color)
  }

  // 根據狀態過濾燈號
  const getStatusLightsByStatus = (status: StatusLight['status']) => {
    return statusLights.value.filter(light => light.status === status)
  }

  // 計算狀態統計
  const statusStats = computed(() => {
    const lights = statusLights.value
    return {
      total: lights.length,
      green: lights.filter(light => light.color === 'green').length,
      yellow: lights.filter(light => light.color === 'yellow').length,
      red: lights.filter(light => light.color === 'red').length,
    }
  })

  // 檢查是否有錯誤狀態
  const hasErrors = computed(() => {
    return statusLights.value.some(light => light.status === 'error')
  })

  // 檢查所有狀態是否正常
  const allNormal = computed(() => {
    return statusLights.value.every(light => light.status === 'running')
  })

  return {
    // 狀態
    statusLights,
    websocketConnected,
    websocketError,
    loading,
    statusStats,
    hasErrors,
    allNormal,
    
    // 操作
    fetchStatusLights,
    refreshStatus,
    connectWebSocket,
    disconnectWebSocket,
    getStatusLightsByColor,
    getStatusLightsByStatus,
  }
})
import { defineStore } from 'pinia'
import { ref } from 'vue'
import { message } from 'ant-design-vue'
import api from '~/utils/api'

interface Function {
  id: string
  name: string
  description?: string
  command: string
  args: string[]
  timeout: number
  status: 'running' | 'stopped' | 'error'
  last_executed?: string
  last_status?: 'success' | 'failed' | 'timeout' | 'stopped'
  created_at: string
  updated_at: string
}

interface CreateFunctionRequest {
  name: string
  description?: string
  command: string
  args?: string[]
  timeout?: number
}

interface UpdateFunctionRequest {
  name?: string
  description?: string
  command?: string
  args?: string[]
  timeout?: number
}

interface ExecuteFunctionRequest {
  args?: string[]
  timeout?: number
}

export const useFunctionStore = defineStore('functions', () => {
  const functions = ref<Function[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)

  // 獲取所有功能
  const fetchFunctions = async (): Promise<Function[]> => {
    loading.value = true
    error.value = null
    
    try {
      const response = await api.get('/functions')
      functions.value = response.data
      return response.data
    } catch (err: any) {
      error.value = err.message || '獲取功能列表失敗'
      message.error(error.value)
      throw err
    } finally {
      loading.value = false
    }
  }

  // 獲取單個功能
  const fetchFunction = async (id: string): Promise<Function> => {
    loading.value = true
    error.value = null
    
    try {
      const response = await api.get(`/functions/${id}`)
      return response.data
    } catch (err: any) {
      error.value = err.message || '獲取功能失敗'
      message.error(error.value)
      throw err
    } finally {
      loading.value = false
    }
  }

  // 創建新功能
  const createFunction = async (data: CreateFunctionRequest): Promise<Function> => {
    loading.value = true
    error.value = null
    
    try {
      const response = await api.post('/functions', data)
      const newFunction = response.data
      
      // 添加到列表
      functions.value.push(newFunction)
      
      message.success('功能創建成功')
      return newFunction
    } catch (err: any) {
      error.value = err.message || '創建功能失敗'
      message.error(error.value)
      throw err
    } finally {
      loading.value = false
    }
  }

  // 更新功能
  const updateFunction = async (id: string, data: UpdateFunctionRequest): Promise<Function> => {
    loading.value = true
    error.value = null
    
    try {
      const response = await api.put(`/functions/${id}`, data)
      const updatedFunction = response.data
      
      // 更新列表中的功能
      const index = functions.value.findIndex(f => f.id === id)
      if (index !== -1) {
        functions.value[index] = updatedFunction
      }
      
      message.success('功能更新成功')
      return updatedFunction
    } catch (err: any) {
      error.value = err.message || '更新功能失敗'
      message.error(error.value)
      throw err
    } finally {
      loading.value = false
    }
  }

  // 刪除功能
  const deleteFunction = async (id: string): Promise<void> => {
    loading.value = true
    error.value = null
    
    try {
      await api.delete(`/functions/${id}`)
      
      // 從列表中移除
      functions.value = functions.value.filter(f => f.id !== id)
      
      message.success('功能刪除成功')
    } catch (err: any) {
      error.value = err.message || '刪除功能失敗'
      message.error(error.value)
      throw err
    } finally {
      loading.value = false
    }
  }

  // 執行功能
  const executeFunction = async (id: string, data: ExecuteFunctionRequest): Promise<any> => {
    loading.value = true
    error.value = null
    
    try {
      const response = await api.post(`/functions/${id}/run`, data)
      
      message.success('功能執行成功')
      return response.data
    } catch (err: any) {
      error.value = err.message || '執行功能失敗'
      message.error(error.value)
      throw err
    } finally {
      loading.value = false
    }
  }

  // 獲取執行記錄
  const getExecutionLogs = async (id: string): Promise<any[]> => {
    loading.value = true
    error.value = null
    
    try {
      const response = await api.get(`/functions/${id}/logs`)
      return response.data
    } catch (err: any) {
      error.value = err.message || '獲取執行記錄失敗'
      message.error(error.value)
      throw err
    } finally {
      loading.value = false
    }
  }

  // 根據狀態過濾功能
  const getFunctionsByStatus = (status: Function['status']) => {
    return functions.value.filter(f => f.status === status)
  }

  return {
    // 狀態
    functions,
    loading,
    error,
    
    // 操作
    fetchFunctions,
    fetchFunction,
    createFunction,
    updateFunction,
    deleteFunction,
    executeFunction,
    getExecutionLogs,
    getFunctionsByStatus,
  }
})
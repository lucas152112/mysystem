import axios from 'axios'
import { message } from 'ant-design-vue'

// 創建 axios 實例
const api = axios.create({
  baseURL: 'http://localhost:3000/api',
  timeout: 10000,
  headers: {
    'Content-Type': 'application/json',
  },
})

// 請求攔截器
api.interceptors.request.use(
  (config) => {
    // 可以在這裡添加認證令牌等
    // const token = localStorage.getItem('token')
    // if (token) {
    //   config.headers.Authorization = `Bearer ${token}`
    // }
    
    // 從 localStorage 獲取配置（或使用環境變量）
    const apiBaseUrl = localStorage.getItem('apiBaseUrl')
    if (apiBaseUrl) {
      config.baseURL = apiBaseUrl
    }
    
    return config
  },
  (error) => {
    return Promise.reject(error)
  }
)

// 響應攔截器
api.interceptors.response.use(
  (response) => {
    return response
  },
  (error) => {
    // 統一錯誤處理
    if (error.response) {
      // 服務器返回錯誤
      const { status, data } = error.response
      
      switch (status) {
        case 400:
          message.error(data.error || '請求參數錯誤')
          break
        case 401:
          message.error('未授權，請重新登入')
          // 可以跳轉到登入頁面
          break
        case 403:
          message.error('權限不足')
          break
        case 404:
          message.error('資源不存在')
          break
        case 500:
          message.error('服務器內部錯誤')
          break
        default:
          message.error(`請求失敗: ${status}`)
      }
    } else if (error.request) {
      // 請求已發送但無響應
      message.error('網絡錯誤，請檢查網絡連接')
    } else {
      // 請求配置錯誤
      message.error('請求配置錯誤')
    }
    
    return Promise.reject(error)
  }
)

// 設置 API 基礎 URL
export const setApiBaseUrl = (url: string) => {
  api.defaults.baseURL = url
  localStorage.setItem('apiBaseUrl', url)
}

export default api
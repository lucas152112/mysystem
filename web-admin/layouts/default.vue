<template>
  <a-layout style="min-height: 100vh">
    <!-- 側邊欄 -->
    <a-layout-sider v-model:collapsed="collapsed" collapsible>
      <div class="logo">
        <h2 v-if="!collapsed">mysystem</h2>
        <h2 v-else>MS</h2>
      </div>
      
      <a-menu
        v-model:selectedKeys="selectedKeys"
        theme="dark"
        mode="inline"
        :items="menuItems"
        @click="handleMenuClick"
      />
    </a-layout-sider>
    
    <!-- 主內容區 -->
    <a-layout>
      <!-- 頂部導航欄 -->
      <a-layout-header style="background: #fff; padding: 0 20px; display: flex; align-items: center; justify-content: space-between;">
        <div>
          <a-breadcrumb style="margin: 16px 0">
            <a-breadcrumb-item>mysystem</a-breadcrumb-item>
            <a-breadcrumb-item>{{ currentPageTitle }}</a-breadcrumb-item>
          </a-breadcrumb>
        </div>
        
        <div style="display: flex; align-items: center; gap: 16px;">
          <!-- 狀態指示器 -->
          <div style="display: flex; align-items: center; gap: 8px;">
            <div style="width: 8px; height: 8px; border-radius: 50%; background: #52c41a;"></div>
            <span>已連接</span>
          </div>
          
          <!-- 用戶資訊 -->
          <a-dropdown>
            <div style="cursor: pointer; display: flex; align-items: center; gap: 8px;">
              <a-avatar style="background-color: #1890ff">Admin</a-avatar>
              <span>管理員</span>
            </div>
          </a-dropdown>
        </div>
      </a-layout-header>
      
      <!-- 內容區域 -->
      <a-layout-content style="margin: 24px 16px; padding: 24px; background: #fff; min-height: 280px;">
        <slot />
      </a-layout-content>
      
      <!-- 頁腳 -->
      <a-layout-footer style="text-align: center;">
        mysystem ©2026 全部系統的一鍵管理應用
      </a-layout-footer>
    </a-layout>
  </a-layout>
</template>

<script setup lang="ts">
import { ref, computed, h, watch } from 'vue'
import { useRoute, useRouter } from '#app'
import {
  DashboardOutlined,
  ControlOutlined,
  SettingOutlined,
  HistoryOutlined,
  BarChartOutlined,
} from '@ant-design/icons-vue'

const route = useRoute()
const router = useRouter()

const collapsed = ref(false)
const selectedKeys = ref<string[]>(['dashboard'])

// 菜單項目
const menuItems = computed(() => [
  {
    key: 'dashboard',
    icon: () => h(DashboardOutlined),
    label: '儀表板',
    title: '儀表板',
  },
  {
    key: 'functions',
    icon: () => h(ControlOutlined),
    label: '功能管理',
    title: '功能管理',
  },
  {
    key: 'monitor',
    icon: () => h(BarChartOutlined),
    label: '狀態監控',
    title: '狀態監控',
  },
  {
    key: 'history',
    icon: () => h(HistoryOutlined),
    label: '執行記錄',
    title: '執行記錄',
  },
  {
    key: 'settings',
    icon: () => h(SettingOutlined),
    label: '系統設定',
    title: '系統設定',
  },
])

// 當前頁面標題
const currentPageTitle = computed(() => {
  const item = menuItems.value.find(item => item.key === selectedKeys.value[0])
  return item?.title || '儀表板'
})

// 處理菜單點擊
const handleMenuClick = ({ key }: { key: string }) => {
  selectedKeys.value = [key]
  router.push(`/${key === 'dashboard' ? '' : key}`)
}

// 監聽路由變化
watch(() => route.path, (path) => {
  const key = path === '/' ? 'dashboard' : path.slice(1)
  selectedKeys.value = [key]
}, { immediate: true })
</script>

<style scoped>
.logo {
  height: 64px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  background: rgba(255, 255, 255, 0.1);
  margin: 16px;
  border-radius: 4px;
}

.logo h2 {
  color: white;
  margin: 0;
  font-size: 18px;
  font-weight: bold;
}
</style>
<template>
  <div>
    <NuxtLayout>
      <NuxtPage />
    </NuxtLayout>
  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import { useStatusStore } from '~/stores/status'

const statusStore = useStatusStore()

onMounted(() => {
  // 初始化 WebSocket 連接
  statusStore.connectWebSocket()
})

onBeforeUnmount(() => {
  // 清理 WebSocket 連接
  statusStore.disconnectWebSocket()
})
</script>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
  background-color: #f0f2f5;
}
</style>
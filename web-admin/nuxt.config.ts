// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  devtools: { enabled: true },
  
  modules: [
    '@pinia/nuxt',
  ],
  
  css: [
    'ant-design-vue/dist/reset.css'
  ],
  
  vite: {
    css: {
      preprocessorOptions: {
        less: {
          javascriptEnabled: true,
        },
      },
    },
  },
  
  build: {
    transpile: ['ant-design-vue'],
  },
  
  app: {
    head: {
      title: 'mysystem - 系統一鍵管理',
      meta: [
        { charset: 'utf-8' },
        { name: 'viewport', content: 'width=device-width, initial-scale=1' },
        { name: 'description', content: '全部系統的一鍵管理應用' },
      ],
      link: [
        { rel: 'icon', type: 'image/x-icon', href: '/favicon.ico' }
      ],
    },
  },
  
  runtimeConfig: {
    public: {
      apiBaseUrl: process.env.API_BASE_URL || 'http://localhost:3000/api',
      websocketUrl: process.env.WEBSOCKET_URL || 'ws://localhost:3000/api/ws',
    },
  },
})
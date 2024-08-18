import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

// https://vitejs.dev/config/
export default defineConfig({
  server: {
    port: 5173,
    host: '0.0.0.0', // 监听所有网络接口
    // strictPort: true, // 严格指定端口，确保只监听指定的端口
    // proxy: {
    //   // 可选的代理设置
    //   // '/api': 'http://localhost:3000',
    // },
  },
  plugins: [vue()],
})
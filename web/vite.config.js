import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

// https://vite.dev/config/
export default defineConfig({
  plugins: [react()],
  base: '/round_robin_tournament/',
  server: {
    fs: {
      allow: ['..']
    }
  }
})

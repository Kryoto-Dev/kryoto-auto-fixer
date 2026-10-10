import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import tailwind from '@tailwindcss/vite'
import { fileURLToPath, URL } from 'node:url'
export default defineConfig({
  plugins: [react(), tailwind()],
  resolve: { alias: { '@': fileURLToPath(new URL('../kryoto-desktop/src', import.meta.url)) }, dedupe: ['react', 'react-dom'] },
  server: { port: 1422, strictPort: true, fs: { allow: ['..'] } },
  clearScreen: false,
})

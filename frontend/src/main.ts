import { createHead } from '@vueuse/head'
import { createPinia } from 'pinia'
import { setupLayouts } from 'virtual:generated-layouts'
import { createApp } from 'vue'
import { createRouter, createWebHistory } from 'vue-router'

import App from './App.vue'
import './assets/main.css'
import generatedRoutes from '~pages'

const routes = setupLayouts(generatedRoutes)
const router = createRouter({
  history: createWebHistory(),
  routes,
})
const head = createHead()
const app = createApp(App)

app.use(createPinia())
app.use(router)
app.use(head)

app.mount('#app')

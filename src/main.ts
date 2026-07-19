import { createPinia } from 'pinia'
import { createApp } from 'vue'
import App from './App.vue'
import 'uno.css'
import './styles/base.css'

createApp(App).use(createPinia()).mount('#app')

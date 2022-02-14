import { createApp } from 'vue'
import App from './App.vue'
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'
import router from "./router/router";
import VXETable from 'vxe-table'
import 'vxe-table/lib/style.css'

createApp(App).use(ElementPlus).use(router).use(VXETable).mount('#app')

import { createApp } from 'vue'
import App from './layout/index.vue'
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'
import router from "./router/router";
import VXETable from 'vxe-table'
import 'vxe-table/lib/style.css'
import "./global_event"

createApp(App).use(ElementPlus).use(router).use(VXETable).mount('#app')

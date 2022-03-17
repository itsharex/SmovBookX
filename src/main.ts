import { createApp } from 'vue'
import App from './layout/index.vue'
import router from "./router/router";
import VXETable from 'vxe-table'
import 'vxe-table/lib/style.css'
import "./global_event"

console.log(process.env);

createApp(App).use(router).use(VXETable).mount('#app');
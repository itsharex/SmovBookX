import { createApp } from 'vue'
import App from './layout/index.vue'
import router from "./router/router";
import VXETable from 'vxe-table'
import 'vxe-table/lib/style.css'
import "./global_event"
import alert from './util/alert'
import 'default-passive-events';

createApp(App).use(router).use(VXETable).use(alert).mount('#app');
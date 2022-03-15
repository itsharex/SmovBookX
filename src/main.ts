import { createApp } from 'vue'
import App from './layout/index.vue'
import router from "./router/router";
import VXETable from 'vxe-table'
import 'vxe-table/lib/style.css'
import "./global_event"

// import devtools from '@vue/devtools'

// if (process.env.NODE_ENV === 'development') {
//   devtools.connect();
// }

createApp(App).use(router).use(VXETable).mount('#app');
import { createApp } from 'vue'
import App from './layout/index.vue'
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'
import router from "./router/router";
import VXETable from 'vxe-table'
import 'vxe-table/lib/style.css'
import "./global_event"
import devtools from '@vue/devtools'
import cors from 'cors'

// const devtool = () => {
//   if (process.env.NODE_ENV === 'development') {
//     devtools.connect();
//     // "localhost", 3000
//   }
// }

// devtool();

createApp(App).use(ElementPlus).use(router).use(VXETable).use(cors).mount('#app');
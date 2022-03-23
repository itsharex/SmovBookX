import { createApp } from 'vue'
import smovDetail from './SmovDetail.vue'
// import smovDetail from '../dialog/dialog.vue'
import ElementPlus from 'element-plus'

interface Option {
  data:any
}

function mountContent(option = {} as Option) {
  const mainDom = document.getElementById("smovMain") || document.body;
  const dom = document.createElement('div')
  dom.className = "domSmovDetail"
  mainDom.appendChild(dom)
  const app = createApp(smovDetail, {
    close: () => { app.unmount(); mainDom.removeChild(dom) },
    ...option
  })
  app.use(ElementPlus).mount(dom)
}
export default mountContent
import { createApp } from 'vue'
import dialog from './dialog.vue'
import ElementPlus from 'element-plus'


interface Option {
  title?: string;
  content?: string;
}

function mountContent(option = {} as Option) {
  const dom = document.createElement('div')
  document.body.appendChild(dom)
  const app = createApp(dialog, {
    close: () => { app.unmount(); document.body.removeChild(dom) },
    ...option
  })
  app.use(ElementPlus).mount(dom)
}
export default mountContent
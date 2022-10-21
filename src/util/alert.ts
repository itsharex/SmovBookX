import { ElMessage } from 'element-plus';
import { BuildPropType } from 'element-plus/es/utils/';

const _alert = (t: BuildPropType<StringConstructor, "info" | "success" | "warning" | "error", unknown> | undefined, msg: any) => {
    ElMessage.closeAll()
    ElMessage({
        message: msg,
        type: t,
        duration: 2000,
        showClose: true,
    })

}

export const alert = {
    success: (msg: any) => _alert('success', msg),
    error: (msg: any) => _alert('error', msg),
    warning: (msg: any) => _alert('warning', msg),
    info: (msg: any) => _alert('info', msg)
}
const install = (app: any) => {
    app.config.globalProperties.$alert = alert
}
export default {
    install
}
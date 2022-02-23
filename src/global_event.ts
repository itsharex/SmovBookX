import {listen} from '@tauri-apps/api/event';
import {getCurrent, WebviewWindow} from '@tauri-apps/api/window';

!(async () => await listen('frontend_log', (event) => {
    console.log(event.payload)
}))()

!(async () => await listen('single', (event) => {
    console.log("get signal")
    let current = getCurrent()
    current.setAlwaysOnTop(true)
    current.show()
    setTimeout(()=>{
        current.setAlwaysOnTop(false)
    },50)
}))()


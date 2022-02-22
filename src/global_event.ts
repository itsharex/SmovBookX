import {listen} from '@tauri-apps/api/event';
import {getCurrent, WebviewWindow} from '@tauri-apps/api/window';

!(async () => await listen('set', (event) => {
    new WebviewWindow('app-set', {
        url: '/#/set',
        title: '软件设置',
        width: 360,
        height: 280,
        center: true,
        visible: false,
        alwaysOnTop: true,
        skipTaskbar: true,
        resizable: false,
    })
}))()
!(async () => await listen('single', (event) => {
    console.log(event.payload)
    let current = getCurrent()
    current.setAlwaysOnTop(true)
    current.show()
    setTimeout(()=>{
        current.setAlwaysOnTop(false)
    },50)
}))()

!(async () => await listen('log', (event) => {
    console.log("event")
}))()
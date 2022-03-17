import { listen } from '@tauri-apps/api/event';
import { getCurrent, WebviewWindow } from '@tauri-apps/api/window';

!(async () => await listen('single', (event) => {
    console.log("get signal")
    let current = getCurrent()
    current.setAlwaysOnTop(true)
    current.show()
    setTimeout(() => {
        current.setAlwaysOnTop(false)
    }, 50)
}))()

// import { emit } from "@tauri-apps/api/event";
// emit("tauri://update");
// console.log("发送检索");

// !(async () => await listen('tauri://update-available', (event) => {
//     console.log(event);

// }))()





import { listen } from '@tauri-apps/api/event';
import { getCurrent, WebviewWindow } from '@tauri-apps/api/window';

let label = getCurrent().label;
console.log(label)

!(async () => await listen(label+'_single', (event) => {
    console.log("aaaa")
    let current = getCurrent()
    current.unminimize()
    current.setAlwaysOnTop(true)
    current.show()
    setTimeout(() => {
        current.setAlwaysOnTop(false)
    }, 50)
}))()





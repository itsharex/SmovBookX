import { invoke, InvokeArgs } from '@tauri-apps/api/tauri'
import { alert } from './alert';

export const request = (eventName: string, params?: InvokeArgs | undefined) => {
    return new Promise((resolve, reject) => {
        invoke(eventName, params)
            .then((res: any) => {
                if (!res) {
                    resolve(res)
                    return
                }
                if (res.code !== 200) {
                    throw res.msg
                }
                resolve(res)
            })
            .catch(e => {
                console.log(e)
                alert.error(e.toString())
                reject(e)
            })
    })
}


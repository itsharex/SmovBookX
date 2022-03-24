<template>
    <div>
        <el-button type="danger" @click="test1">cs</el-button>
        <el-button type="danger" @click="test2">csss</el-button>
        <el-button type="danger" @click="test3">文件检索测试</el-button>
        <el-button type="danger" @click="test4">即时渲染测试</el-button>
        <el-button type="danger" @click="test5">打开界面测试</el-button>
        <router-view></router-view>
    </div>
</template>

<script lang="ts">
import { defineComponent, reactive, ref, inject, h, render, createApp, Transition, withCtx } from 'vue'
import { ThreadPool } from '../ts/ThreadPool'
import { useRouter } from 'vue-router';
import { Log } from '../type/log'
import { CurentTime } from '../util/time'
import { dialog, notification } from '@tauri-apps/api';
import { readDir } from '@tauri-apps/api/fs';
import { convertFileSrc } from '@tauri-apps/api/tauri';
import { WebviewWindow } from '@tauri-apps/api/window';

export default defineComponent({
    setup() {
        // 任务实体
        const router = useRouter();

        const test = () => {

        }

        const src = ref(convertFileSrc("E:\\ccccc\\zl\\SSNI-107\\img\\thumbs_SSNI-107.jpg"))

        let logs = inject('log') as any;

        const test1 = () => {
            // router.push({
            //     path: '/test2',
            //     query: {
            //         num: 1
            //     }
            // })
            // dialog.message("fuck")

            const option: notification.Options = {
                title: "test",
                body: "test",
                icon: "test",
            }

            notification.sendNotification(option);
        }

        const test2 = () => {
            const log: Log = {
                time: CurentTime(),
                thread: '前端发消息的测试',
                msg: '前端发消息的测试',
                level: 'INFO'
            }
            logs.value = log;
        }

        const test3 = () => {
            // readDir("E:\\ccccc\\zl\\SSNI-107\\img").then( //只能打开 几个基础文件夹的文件 不然不让打开 放弃这个地方了
            //     (res: any) => {
            //         console.log(res)
            //     }
            // )

            const assetUrl = convertFileSrc("E:\\ccccc\\zl\\SSNI-107\\img\\thumbs_SSNI-107.jpg");
            console.log(assetUrl)
        }


        const test4_1 = {
            name: "test4_1",
            setup() {
                return () => {
                    return h(
                        // 1. 要渲染的标签名称：第一个参数【必需】
                        "button",
                        // 2. 渲染标签的属性：第二个参数【可选】
                        {
                            style: {
                                color: "#333",
                                border: "1px solid #ccc",
                            },
                            class: "wdnmd",
                            id: "",
                            onClick: changeNum,
                        },
                        // 3. 渲染标签的子元素数组：第三个参数【可选】
                        [
                            // "render函数文本" 文本内容可以直接写入
                            h("p", num.value), // h()创建的VNodes
                        ]
                    );
                }
            }
        };

        let num = ref(0);  // vue3中需要使用ref或reactive声明变量，否则无法实现双向数据绑定
        const changeNum = () => {
            num.value++    // 改变ref定义的变量值需要使用.value , reactive不需要 
        }



        const test4 = () => {
            createApp(test4_1).mount(document.createElement("div"));
        }



        const test5 = async () => {

            // loading embedded asset:
            const webview = new WebviewWindow('SSNI-126', {
                url: '/SmovDetail/1'
            })


            webview.once('tauri://created', function () {
                // webview window successfully created
            })
            webview.once('tauri://error', function (e) {
                // an error happened creating the webview window
            })

            // emit an event to the backend
            await webview.emit("some event", "data")
            // listen to an event from the backend
            const unlisten = await webview.listen("event name", e => { })
            unlisten()

            // webview.show();

        }



        return {
            test,
            test1,
            test2,
            test3,
            test4,
            test5,
            src
        }
    }
})

</script>

<style>
</style>
<template>
    <div>
        <el-button type="danger" @click="test1">cs</el-button>
        <el-button type="danger" @click="test2">csss</el-button>
        <el-button type="danger" @click="test3">文件检索测试</el-button>
        <img :src="src" alt="">
        <router-view></router-view>
    </div>
</template>

<script lang="ts">
import { defineComponent, reactive, ref, inject } from 'vue'
import { ThreadPool } from '../ts/ThreadPool'
import { useRouter } from 'vue-router';
import { Log } from '../type/log'
import { CurentTime } from '../util/time'
import { dialog, notification } from '@tauri-apps/api';
import { readDir } from '@tauri-apps/api/fs';
import { convertFileSrc } from '@tauri-apps/api/tauri';

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



        return {
            test,
            test1,
            test2,
            test3,
            src
        }
    }
})
</script>

<style>
</style>
<template>
    <div>
        <el-button type="danger" @click="test1">cs</el-button>
        <el-button type="danger" @click="test2">csss</el-button>
        <router-view></router-view>
    </div>
</template>

<script lang="ts">
import { defineComponent, reactive, ref, inject } from 'vue'
import { ThreadPool } from '../ts/ThreadPool'
import { useRouter } from 'vue-router';
import { Log } from '../type/log'
import { CurentTime } from '../util/time'

export default defineComponent({
    setup() {
        // 任务实体
        const router = useRouter();

        const test = () => {

        }

        let logs = inject('log') as any;

        const test1 = () => {
            router.push({
                path: '/test2',
                query: {
                    num: 1
                }
            })
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



        return {
            test,
            test1,
            test2
        }
    }
})
</script>

<style>
</style>
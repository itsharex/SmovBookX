<template>
    <div>
        <el-button type="danger" @click="test1">cs</el-button>
        <router-view></router-view>
    </div>
</template>

<script lang="ts">
import { defineComponent, reactive, ref } from 'vue'
import { ThreadPool } from '../ts/ThreadPool'
import { useRouter } from 'vue-router';

export default defineComponent({
    setup() {
        // 任务实体
        const router = useRouter();

        const test = () => {

        }

        const test1 = () => {
            router.push({
                path: '/test2',
                query: {
                    num: 1
                }
            })
        }



        return {
            test,
            test1
        }
    }
})
</script>

<style>
</style>
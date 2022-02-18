<template>
    <div>
        <el-button type="danger" @click="test">cs</el-button>
    </div>
</template>

<script lang="ts">
import { defineComponent, reactive, ref } from 'vue'
import {ThreadPool} from '../ts/ThreadPool'

export default defineComponent({
    setup() {
        // 任务实体

        function genTaskObj(i) {
            return new ThreadPool.Task({
                params: i,
                processor: (params) => {
                    // console.log("线程"+i+"正在运行");
                    return new Promise(resolve => {
                        setTimeout(() => {
                            resolve(params);
                        }, 1000);
                    });
                },
                callback: (data) => {
                    console.log(`线程 ${i}, rst is`, data);
                }
            });
        };



        const test = () => {
            // const tasks7 = [
            //     genTaskObj(1),
            //     genTaskObj(2),
            //     genTaskObj(3),
            // ];

            let tasks7:any[] = [];

            for (let i = 1 ; i<100 ; i++){
                tasks7.push(
                    genTaskObj(i)
                )
            }
            let pool7 = new ThreadPool.FixedThreadPool({
                size: 10,
                tasks: [...tasks7] //, ...tasks7, ...tasks7
            })
            pool7.start();

            // setTimeout(() => {
            //     pool7.stop();
            // }, 3000); // 3s后停止
        }



        return {
            test
        }
    }
})
</script>

<style>
</style>
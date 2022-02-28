<template>
    <div>
        <el-button @click="click" color="#626aef" style="color: white">Custom</el-button>
        <el-button @click="click1" color="#626aef" style="color: rgb(255, 0, 0)">Custom1</el-button>
        <el-button @click="click2" color="#626aef" style="color: rgb(0, 255, 221)">Custom2</el-button>

        <div v-for="(item, index) in pool.tasks" :key="index" style="line-height: auto">
            <el-button type="success">{{ item.params.seekName }}</el-button>
        </div>
    </div>
</template>

<script lang='ts'>
import { defineComponent, ref, reactive } from 'vue';
import { ThreadPool } from '../ts/ThreadPool';
import { invoke } from "@tauri-apps/api/tauri";
export default defineComponent({
    name: 'Seek',
    props: [],
    setup(props, { emit }) {

        // let tasks: any[] = [];

        let pool = reactive(new ThreadPool.FixedThreadPool({
            size: 1,
            tasks: [],
        }))

        const click = () => {
            pool.tasks.push(retrieveData("asdasd", 1));
            console.log(pool.tasks)
        }

        const click1 = () => {
            pool.start();
        }

        const click2 = () => {
            pool.stop();
        }

        function retrieveData(seekName, id) {
            const params = {
                seekName: seekName,
                smovId: id,
                status: 0  //0 未在检索状态 1正在检索 2检索完成
            }
            return new ThreadPool.Task({
                params: params,
                processor: (params) => {
                    // console.log("线程"+i+"正在运行");
                    return new Promise(resolve => {

                        setTimeout(() => {
                            console.log("正在检索", seekName);
                            resolve(params);
                        }, 5000);

                        // invoke("retrieve_data",params).then((res) => {
                        //     console.log(res);
                        // }).finally(() => {
                        //     resolve(params);
                        // });

                    });
                },
                callback: (data) => {
                    console.log(`线程 ${pool.tasks.length}, rst is`, data);
                }
            });
        }
        return {
            click,
            click1,
            click2,
            pool

        };
    }
})

</script>
<style lang='less' scoped>
</style>
<template>
    <div>
        <el-button @click="click" color="#626aef" style="color: white">Custom</el-button>
        <el-button @click="click1" color="#626aef" style="color: rgb(255, 0, 0)">Custom1</el-button>
        <el-button @click="click2" color="#626aef" style="color: rgb(0, 255, 221)">Custom2</el-button>

        <div class="testDiv">
            <div v-for="(item, index) in pool.tasks" :key="index">
                <el-button type="success" :loading="item.params.status == 1">{{ item.params.id }}</el-button>
            </div>
        </div>
    </div>
</template>

<script lang='ts'>
import { defineComponent, ref, reactive, inject } from 'vue';
import { ThreadPool } from '../ts/ThreadPool';
import { invoke } from "@tauri-apps/api/tauri";
export default defineComponent({
    name: 'Seek',
    props: [],
    setup(props, { emit }) {

        let i = 1;

        let pool = reactive(new ThreadPool.FixedThreadPool({
            size: 2,
            tasks: [],
            runningFlag: inject("seek")
        }))

        const click = () => {
            pool.addTask(retrieveData("asdasd", i));
            i++;
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
                id: id,
                status: 0
            }
            return new ThreadPool.Task({
                params: params,
                processor: (params) => {
                    return new Promise(resolve => {
                        setTimeout(() => {
                            // console.log("正在检索", seekName);
                            resolve(params);
                        }, 2000);

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
.testDiv {
    display: flex;
    flex-wrap: wrap;
    line-height: 12px;
    * {
        margin: 5px;
    }
}
</style>
<template>
    <div class="seek">
        <div class = "buttonDiv">
            <el-button @click="click" color="#626aef" style="color: white">添加测试元素</el-button>
            <el-button @click="click1" color="#626aef" style="color: rgb(255, 255, 255)">开始检索</el-button>
            <el-button @click="click2" color="#626aef" style="color: rgb(255, 255, 255)">停止检索</el-button>
            <el-button @click="click3" color="#626aef" style="color: rgb(255, 255, 255)">关闭窗口</el-button>
        </div>

        <div class="testDiv">
            <div v-for="(item, index) in pool.tasks" :key="index">
                <el-button type="success" :loading="item.params.status == 1">{{ item.params.id }}</el-button>
            </div>
        </div>
    </div>
</template>

<script lang='ts'>
import { defineComponent, ref, reactive, inject, watch, getCurrentScope } from 'vue';
import { ThreadPool } from '../ts/ThreadPool';
import { invoke } from "@tauri-apps/api/tauri";
import { getAll, getCurrent } from '@tauri-apps/api/window';
export default defineComponent({
    name: 'Seek',
    props: [],
    setup(props, { emit }) {

        let i = 1;

        let pool = reactive(new ThreadPool.FixedThreadPool({
            size: 1,
            tasks: [],
            runningFlag: false,
            autoRun: false
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

        const click3 = () => {
            getCurrent().hide();
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
                    // console.log(`线程 ${pool.tasks.length}, rst is`, data);
                }
            });
        }
        return {
            click,
            click1,
            click2,
            click3,
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

.seek {
    // width: 100%;
    // height: 100%;
    // text-align: center;
    // background-color: #fffae8;
    // border-radius: 30px;
    // box-shadow: 8px 8px 10px grey;
    // -webkit-app-region: drag;
}

.buttonDiv{
    display: flex;
    justify-content: center;
    flex-wrap: wrap;
    align-items: center;
    * {
        margin: 3px;
    }
}
</style>
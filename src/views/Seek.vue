<template>
    <div class="seek">
        <div class="buttonDiv">
            <el-button @click="click" color="#626aef" style="color: white">添加测试元素</el-button>
            <el-button @click="click1" color="#626aef" style="color: rgb(255, 255, 255)">开始检索</el-button>
            <el-button @click="click2" color="#626aef" style="color: rgb(255, 255, 255)">停止检索</el-button>
            <el-button @click="click3" color="#626aef" style="color: rgb(255, 255, 255)">关闭窗口</el-button>
        </div>

        <div class="smovList">
            <div v-for="(item, index) in pool.tasks" :key="index" class="smov">
                <!-- v-loading="item.params.status == 3" -->
                <el-card
                    class="smovCard"
                    :class="item.params.status == 1 ? 'smovCard_suss' : item.params.status == 2 ? 'smovCard_fail' : item.params.status == 3 ? 'smovCard_seeking' : ''"
                >
                    <div class="smovName">{{ item.params.seekName }}</div>
                    <!-- -->
                    <div class="loadingDiv" v-if="item.params.status == 3">
                        <el-icon color="#409EFC" class="is-loading loading">
                            <loading />
                        </el-icon>
                    </div>

                    <div class="close">
                        <el-button
                            type="text"
                            :icon="Delete"
                            circle
                            @click="deleteTask(index, item.params.id, 'normal')"
                        ></el-button>
                    </div>
                </el-card>
            </div>
        </div>
    </div>
</template>

<script lang='ts'>
import { defineComponent, ref, reactive, inject, watch, getCurrentScope, onMounted } from 'vue';
import { ThreadPool } from '../ts/ThreadPool';
import { invoke, } from "@tauri-apps/api/tauri";
import { getAll, getCurrent } from '@tauri-apps/api/window';
import { listen } from '@tauri-apps/api/event';
import { Loading, Delete } from '@element-plus/icons-vue';
import { ElMessage } from 'element-plus';
export default defineComponent({
    name: 'Seek',
    components: { Loading },
    props: [],
    setup(props, { emit }) {

        let i = 1;

        const openStatus = ref({
            none: false,
            suss: false,
            fail: false
        })

        let pool = reactive(new ThreadPool.FixedThreadPool({
            size: 1,
            tasks: [],
            runningFlag: false,
            autoRun: false
        }))

        //获取一个检索队列
        const addTaskEvent = () => {
            !(async () => await listen('addTask', (event: any) => {
                event.payload.forEach(item => {
                    pool.addTask(retrieveData(item.seek_name, item.id));
                });
            }))()
        }

        const getSeekSmov = () => {
            invoke("get_seek_smov").then((res: any) => {
                res.data.forEach(item => {
                    pool.addTask(retrieveData(item.seek_name, item.id));
                });
            })
        }

        onMounted(() => {
            addTaskEvent();
            getSeekSmov();
        })

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

        const deleteTask = (index: number, id: number, type: String) => {
            invoke("remove_smov_seek_status", { id: id }).then((res: any) => {
                if (res.code == 200) {
                    pool.removeTask(index, type);
                } else {
                    ElMessage.error('移除检索队列出现错误');
                    return;
                }
            });
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
                            resolve(1);
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
            pool,
            openStatus,
            Delete,
            deleteTask
        };
    }
})

</script>
<style lang='less'>
.testDiv {
    display: flex;
    flex-wrap: wrap;
    line-height: 12px;
    * {
        margin: 5px;
    }
}

.buttonDiv {
    display: flex;
    justify-content: center;
    flex-wrap: wrap;
    align-items: center;
    * {
        margin: 3px;
    }
}

.smov {
    padding: 8px;
    height: 30px;
}

.smovCard {
    width: 100%;
    height: 40px;
    line-height: 40px;
    .el-card__body {
        padding: 0;
        font-size: 14px;
        font-weight: 600;
        position: relative;
    }
}

.smovCard_suss {
    background: #e4ffef;
}

.smovCard_fail {
    background: #ffe0e0;
}

.smovCard_seeking {
}

.loadingDiv {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 100%;
    height: 40px;
    position: absolute;
    top: 0px;
    left: 0px;
    background: #ffffffe6;
}

.loading {
    width: 20px;
    height: 20px;

    svg {
        width: 20px;
        height: 20px;
    }
}

.close {
    position: absolute;
    top: 0px;
    right: 5px;
    display: flex;
    align-items: center;
    height: 100%;
}
</style>
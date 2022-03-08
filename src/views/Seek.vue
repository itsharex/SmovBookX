<template>
    <div class="seek">
        <div class="buttonDiv">
            <el-button @click="click" color="#626aef" style="color: white">添加测试元素</el-button>
            <el-button @click="click1" color="#626aef" style="color: rgb(255, 255, 255)">开始检索</el-button>
            <el-button @click="click2" color="#626aef" style="color: rgb(255, 255, 255)">停止检索</el-button>
            <el-button @click="click3" color="#626aef" style="color: rgb(255, 255, 255)">关闭窗口</el-button>

            <el-button
                @click="openStatus[2] = !openStatus[2]"
                color="#626aef"
                style="color: rgb(255, 255, 255)"
            >错误是否可见</el-button>
            <el-button
                @click="openStatus[1] = !openStatus[1]"
                color="#626aef"
                style="color: rgb(255, 255, 255)"
            >成功是否可见</el-button>
            <el-button
                @click="openStatus[3] = !openStatus[3]"
                color="#626aef"
                style="color: rgb(255, 255, 255)"
            >正在检索是否可见</el-button>
            <el-button
                @click="openStatus[0] = !openStatus[0]"
                color="#626aef"
                style="color: rgb(255, 255, 255)"
            >未检索是否可见</el-button>

            <el-button
                @click="removeAll"
                color="#626aef"
                style="color: rgb(255, 255, 255)"
            >超级牛逼之一键删除</el-button>
        </div>

        <div class="smovList">
            <div v-for="(item, index) in pool.tasks" :key="index">
                <div class="smov" v-if="openStatus[item.params.status] == true">
                    <el-card
                        class="smovCard"
                        :class="item.params.status == 1 ? 'smovCard_suss' : item.params.status == 2 ? 'smovCard_fail' : item.params.status == 3 ? 'smovCard_seeking' : ''"
                    >
                        <div class="smovName">{{ item.params.seek_name }}</div>
                        <div class="loadingDiv" v-if="item.params.status == 3">
                            <el-icon color="#409EFC" class="is-loading loading">
                                <loading />
                            </el-icon>
                        </div>

                        <div class="close">
                            <el-button
                                type="text"
                                v-if="item.params.status != 3"
                                :icon="Delete"
                                circle
                                @click="deleteTask(index, item.params.id)"
                            ></el-button>
                        </div>
                    </el-card>
                </div>
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
import XEUtils from 'xe-utils';
export default defineComponent({
    name: 'Seek',
    components: { Loading },
    props: [],
    setup(props, { emit }) {

        let i = 1;

        const openStatus = ref({
            0: true,  //wait //不应该是判断检索状态 应该是判断检索结果！
            1: true,  //suss
            2: true,  //fail
            3: true  //run time
        })

        let pool = reactive(new ThreadPool.FixedThreadPool({
            size: 1,
            tasks: [],
            runningFlag: false,
            autoRun: false
        }))

        //获取检索队列
        const addTaskEvent = () => {
            !(async () => await listen('addTask', (event: any) => {
                console.log(event);
                event.payload.forEach(item => {
                    pool.addTask(retrieveData(item));
                });
            }))()
        }

        const getSeekSmov = () => {
            invoke("get_seek_smov").then((res: any) => {
                if (res.data) {
                    res.data.forEach(item => {
                        console.log(item);
                        pool.addTask(retrieveData(item));
                    });
                }

            })
        }

        const removeAll = () => {
            XEUtils.arrayEach(pool.tasks, (item, key) => {
                deleteTask(0, item.params.id);
            })
        }

        onMounted(() => {
            addTaskEvent();
            getSeekSmov();
        })

        const randomBoolean = () => Math.random() >= 0.5;

        const click = () => {
            const test = {

            }
            // pool.addTask(retrieveData("asdasd", i));
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

        const deleteTask = (index: number, id: number) => {
            invoke("remove_smov_seek_status", { id: id }).then((res: any) => {
                if (res.code == 200) {
                    XEUtils.throttle(pool.removeTask(index) as any, 100)
                } else {
                    ElMessage.error('移除检索队列出现错误');
                    return;
                }
            });
        }

        function retrieveData(item: any) {

            return new ThreadPool.Task({
                params: item,
                processor: (params) => {
                    return new Promise(resolve => {
                        invoke("retrieve_data", { retrievingSmov: params }).then((res: any) => {
                            if (res.code == 200) {
                                resolve(1);
                            } else {
                                resolve(2);
                            }
                        });
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
            deleteTask,
            removeAll
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

.smovName {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    width: 80%;
    text-align: left;
    margin-left: 10px;
}
</style>
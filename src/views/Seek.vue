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
            <!-- 大数据时有严重的渲染问题 考虑使用vxe重写这个块 或者 自己写一个异步的加入线程 一百条一百条加 -->
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
import { defineComponent, ref, reactive, inject, watch, getCurrentScope, onMounted, onUpdated, nextTick } from 'vue';
import { ThreadPool } from '../ts/ThreadPool';
import { invoke, } from "@tauri-apps/api/tauri";
import { getAll, getCurrent } from '@tauri-apps/api/window';
import { listen } from '@tauri-apps/api/event';
import { Loading, Delete } from '@element-plus/icons-vue';
import { ElMessage, ElLoading } from 'element-plus';
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

        // onUpdated(() => {
        //     scrollToBottom();
        // })

        // const scrollToBottom = () => {
        //     nextTick(() => {
        //         loading.value.visible.value = false;
        //     }
        //     )
        // }



        //获取检索队列
        const addTaskEvent = () => {
            !(async () => await listen('addTask', (event: any) => {
                console.log("检测到数据")
                console.log(Date.now())
                let s = [] as any[];
                event.payload.forEach((item: any) => {
                    pool.addTask(retrieveData(item));
                });
                nextTick(() => {
                    // loading.close();
                    console.log("数据加载完成")
                    console.log(Date.now())
                }
                )
            }))()
        }

        const getSeekSmov = () => {
            invoke("get_seek_smov").then((res: any) => {
                if (res.data) {
                    res.data.forEach((item: any) => {
                        pool.addTask(retrieveData(item));
                    });
                }

            })
        }

        const array_diff = ($array_1: any[], $array_2: any[]) => {
            //将array2转化为一个Object key名为id 值名为undifind 
            let array_2 = {};
            XEUtils.arrayEach($array_2, (item) => {
                array_2[item.params.id] = 1;
            });
            //通过判断Obj中是否有相应key的对象 过滤出需要的
            const newArr = XEUtils.filter($array_1, item => array_2[item.params.id] != 1);

            // console.log(newArr)

            return $array_1;
        }

        onMounted(() => {
            addTaskEvent();
            getSeekSmov();
        });

        const removeAll = () => {
            // XEUtils.arrayEach(pool.tasks, (item, key) => {
            //     deleteTask(0, item.params.id);
            // })
            // const getAllHistory = async (selectRecords) => {
            //     let allHistory = [] as any[];
            //     XEUtils.arrayEach(selectRecords, (item) => {
            //       allHistory.push((async (item) => {
            //             return await getChangePromise(item)
            //         })(item))
            //     });

            //     return await Promise.all(allHistory);
            // };

            // const getChangePromise = (row: any) => {
            //     return new Promise(function (resolve, reject) {
            //         invoke("change_active_status", { id: row.id, status: 0 }).then((res: any) => {
            //             if (res.code == 200) {
            //                 // const $table = xTable.value;
            //                 // $table.remove(row);
            //                 // XEUtils.remove(FileData, toitem => toitem === row)
            //             } else {
            //                 ElMessage.error('关闭出现了一个问题' + res.msg)
            //             }
            //         }).finally(() => {
            //             resolve(row);
            //         })
            //     });
            // }

            // loading.value.visible.value = true;

            const data = XEUtils.map(pool.tasks, item => item.params.id);

            invoke("remove_smov_seek_status", { id: data }).then((res: any) => {
                if (res.code == 200) {
                    ElMessage({
                        message: '将' + data.length + '条数据移出队列',
                        type: 'success',
                    })
                    pool.tasks = [];
                    pool.index = 0;
                    // XEUtils.remove(pool.tasks, item => item.id === id)
                } else {
                    ElMessage.error('移除检索队列出现错误');
                    return;
                }
            });


        }

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
            pool.tasks[index].params.status = 3;
            invoke("remove_smov_seek_status", { id: [id] }).then((res: any) => {
                if (res.code == 200) {

                    pool.removeTask(index);
                    // XEUtils.remove(pool.tasks, item => item.id === id)
                } else {
                    ElMessage.error('移除检索队列出现错误');
                    return;
                }
            });
        }

        function retrieveData(item: any) {

            return new ThreadPool.Task({
                params: item,
                processor: (params: any) => {
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
                callback: (data: any) => {
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
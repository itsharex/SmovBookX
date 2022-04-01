<template>
    <el-container>
        <!-- 检索页面悬浮球 四月份 -->
        <el-header class="header" height="40px">
            <action-bar :imize="false" :minImize="false" :top="true" />
        </el-header>
        <el-main class="main">
            <!-- 版本3 删除会出现卡死现象  方案为 数组队列和虚拟渲染的界面 -->
            <div class="seek">
                <div class="settingDiv">
                    <div class="buttonDiv">
                        <el-button
                            @click="start"
                            color="#C7415B"
                            type="danger"
                            :disabled="pool.delLoading || pool.isRunning()"
                        >开始检索</el-button>
                        <el-button
                            @click="stop"
                            color="#C7415B"
                            type="danger"
                            :loading="pool.delLoading"
                            :disabled="!pool.isRunning()"
                        >停止检索</el-button>
                        <el-button @click="getSeekSmov" color="#C7415B" type="danger">重载数据</el-button>
                        <el-button @click="close" color="#C7415B" type="danger">关闭窗口</el-button>
                        <el-button @click="removeAll" color="#C7415B" type="danger">雁过不留痕风过不留声</el-button>
                    </div>

                    <div class="filtersDiv">
                        <p>
                            错误
                            <el-switch v-model="openStatus[2]" @change="ErrChange" />
                        </p>
                        <p>
                            成功
                            <el-switch v-model="openStatus[1]" @change="SussChange" />
                        </p>
                        <p>
                            未检索
                            <el-switch v-model="openStatus[0]" @change="WaitChange" />
                        </p>
                        <p class="status">当前检索状态:{{ pool.isRunning() ? '是' : '否' }}</p>
                    </div>
                </div>

                <div v-if="HotLoading" class="load">
                    <span>Loading...</span>
                </div>

                <div class="smovList">
                    <!-- 
              大数据时有严重的渲染问题 考虑使用vxe重写这个块 或者 自己写一个异步的加入线程 一百条一百条加  
              测试发现四千条数据的传输时间已经到了300ms 这个速度非常不满意 对于用户可能要做 表格loading 加 分批传输 加 进度条的的功能
              但是进度条还有个问题 渲染是个异步的过程 在渲染时很可能会出现 几百条数据一次性 突然出现 这个时肯定的 有没有其他办法优化用户的体验
              
              当前方案
              1.在数据进入时就给一个 左上角的 loading 代表数据正在进入
              2.压缩传入的数据  传入的数据时间 至少应该要控制在 200ms内
              3.忽略用户感受

              周末优化
              1.将列表用vxe实现 优化性能问题 
              2.优化vxe 外观 ，包括loading等待的界面 
              3.传入数据时增加异步loading状态

              4.线程池不存方法，方法在每次用的时候生成一个 
                    -->
                    <vxe-table
                        border="none"
                        show-overflow
                        resizable
                        keep-source
                        height="100%"
                        :loading="pool.loading"
                        ref="Tasks"
                        :row-config="{ isHover: false, height: 63 }"
                        :show-header="false"
                    >
                        <template #empty>
                            <el-empty style="line-height:50px" description="没有其他数据了哦"></el-empty>
                        </template>
                        <vxe-column
                            field="is_active"
                            title="对象"
                            align="center"
                            class-name="smovColumn"
                        >
                            <template #default="{ row }" class="smovColumn">
                                <div class="smov">
                                    <div
                                        class="smovCard"
                                        :class="row.status == 1 ? 'smovCard_suss' : row.status == 2 ? 'smovCard_fail' : row.status == 3 ? 'smovCard_seeking' : ''"
                                    >
                                        <div class="smovName">{{ row.seek_name }}</div>
                                        <div class="loadingDiv" v-if="row.status == 3">
                                            <el-icon color="#409EFC" class="is-loading loading">
                                                <loading />
                                            </el-icon>
                                        </div>

                                        <div class="close">
                                            <el-button
                                                type="text"
                                                @click="deleteTask(row)"
                                                v-if="row.status != 3"
                                                :icon="Delete"
                                                circle
                                            ></el-button>
                                        </div>
                                    </div>
                                </div>
                            </template>
                        </vxe-column>

                        <vxe-column
                            field="status"
                            :visible="false"
                            :filters="[{ label: '错误', value: 2, checked: openStatus[2] }, { label: '等待', value: 0, checked: openStatus[0] }, { label: '成功', value: 1, checked: openStatus[1] }, { label: '正在执行', value: 3, checked: openStatus[3] }]"
                        ></vxe-column>
                    </vxe-table>
                </div>

                <div class="zw"></div>
            </div>
        </el-main>
    </el-container>
</template>

<script lang='ts'>
import { defineComponent, ref, reactive, inject, watch, getCurrentScope, onMounted, onUpdated, nextTick } from 'vue';
import { ThreadPool } from '../ts/ThreadPool';
import { invoke, } from "@tauri-apps/api/tauri";
import { getAll, getCurrent } from '@tauri-apps/api/window';
import { listen } from '@tauri-apps/api/event';
import { Loading, Delete } from '@element-plus/icons-vue';
import { ElMessage, ElLoading } from 'element-plus';
import 'element-plus/es/components/message/style/css'
import 'element-plus/es/components/loading/style/css'
import XEUtils from 'xe-utils';
import { VXETable, VxeTableInstance, VxeTableEvents, RecordInfo, VxeColumnPropTypes } from "vxe-table";
export default defineComponent({
    name: 'Seek',
    components: { Loading },
    props: [],
    setup(props, { emit }) {

        const Tasks = ref({} as VxeTableInstance)

        const HotLoading = ref(false);

        const openStatus = ref([
            true,  //wait 
            true,  //suss
            true,  //fail
            true,  //run time
            true   //delete run time
        ])

        let pool = reactive(new ThreadPool.FixedThreadPool({
            size: 1,
            runningFlag: false,
            autoRun: false
        }))

        onUpdated(() => {

        })

        const filterStatusMethod: VxeColumnPropTypes.FilterMethod = ({ value, row }) => {
            console.log(value)
            return openStatus.value[row.status]
        }

        const addTaskEvent = () => {
            !(async () => await listen('addTask', (event: any) => {
                HotLoading.value = true;
                console.log(Date.now());
                asyncJoin(event.payload)
            }))()
        }

        const asyncJoin = async (list: any[]) => {
            pool.addTasks(list);
            Tasks.value.reloadData(pool.tasks).then(() => {
                setTimeout(() => {
                    HotLoading.value = false;
                }, 200);

            });


            console.log(pool.tasks.length);
            console.log(Tasks.value.getData().length);

        }

        const getSeekSmov = () => {
            pool.loading = true;
            pool.tasks = [];
            Tasks.value.remove();
            invoke("get_seek_smov").then((res: any) => {
                let data: any = res;
                if (data.code == 200) {
                    pool.addTasks(data.data);
                    //获取最后的索引下标
                    let index = XEUtils.findIndexOf(data.data, item => item.status === 0);
                    if (index == -1) {
                        pool.index = 0;
                    } else {
                        pool.index = index;
                    }
                }
            }).finally(() => {
                const $table = Tasks.value;
                if ($table) {
                    const data = pool.tasks;
                    $table.loadData(data);
                }
                pool.loading = false;
            })
        }

        onMounted(() => {
            nextTick(() => {
                getSeekSmov();
            })
            addTaskEvent();
        });

        const getFilter = () => {
            return [
                { label: '等待', value: 0, checked: openStatus.value[0] },
                { label: '成功', value: 1, checked: openStatus.value[1] },
                { label: '错误', value: 2, checked: openStatus.value[2] },
                { label: '正在执行', value: 3, checked: openStatus.value[3] }
            ];
        }

        const ErrChange = (val: any) => {
            const $table = Tasks.value
            openStatus.value[2] = val;
            const column = $table.getColumnByField('status')
            if (column) {
                const filter = getFilter();
                $table.setFilter(column, filter)
                $table.updateData()
            }
        }

        const SussChange = (val: any) => {
            const $table = Tasks.value
            openStatus.value[1] = val;
            const column = $table.getColumnByField('status')
            if (column) {
                const filter = getFilter();
                $table.setFilter(column, filter)
                $table.updateData()
            }
        }

        const WaitChange = (val: any) => {
            const $table = Tasks.value
            openStatus.value[0] = val;
            const column = $table.getColumnByField('status')
            if (column) {
                const filter = getFilter();
                $table.setFilter(column, filter)
                $table.updateData()
            }
        }

        const removeAll = () => {

            pool.loading = true;

            const data: number[] = XEUtils.map(pool.tasks, item => item.id);

            invoke("remove_smov_seek_status", { id: data }).then((res: any) => {
                if (res.code == 200) {
                    ElMessage({
                        showClose: true,
                        message: '将' + data.length + '条数据移出队列',
                        type: 'success',
                    })
                    Tasks.value.remove();
                    pool.tasks = [];
                    pool.index = 0;
                } else {
                    ElMessage.error('移除检索队列出现错误');
                    return;
                }
            }).finally(() => {
                pool.loading = false;
            });
        }


        const start = () => {
            pool.start();
        }

        const stop = () => {
            pool.stop();
        }

        const close = () => {
            getCurrent().hide();
        }

        const deleteTask = (row: { status: number; id: any; }) => {
            row.status = 3;
            invoke("remove_smov_seek_status", { id: [row.id] }).then((res: any) => {
                if (res.code == 200) {
                    const $table = Tasks.value;
                    let index = $table.getRowIndex(row);
                    $table.remove(row);
                    XEUtils.remove(pool.tasks, item => item.id === row.id);
                    if (index + 1 <= pool.index) {
                        pool.index--;
                    }
                } else {
                    ElMessage.error('移除检索队列出现错误');
                    return;
                }
            });
        }

        return {
            start,
            stop,
            close,
            pool,
            openStatus,
            Delete,
            deleteTask,
            removeAll,
            HotLoading,
            getSeekSmov,
            Tasks,
            ErrChange,
            SussChange,
            WaitChange,
            filterStatusMethod
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

#app {
    height: 100vh;
    .el-container {
        height: 100%;
    }
}

.smov {
    padding: 6px;
    width: 100%;
}

.smovCard {
    width: 100%;
    height: 40px;
    line-height: 40px;
    border-radius: 5px;
    box-shadow: 0px 0px 9px rgba(0, 0, 0, 0.12);
    font-weight: 600;
    padding: 1px;
    .el-card__body {
        padding: 0;
        font-size: 14px;
        font-weight: 600;
        position: relative;
    }
}

.smovCard_suss {
    background: #d1edc4;
}

.smovCard_fail {
    background: #fcd3d3;
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

.load {
    position: fixed;
    top: 0;
    left: 0;
    background: #ffe0e0;
}

.filtersDiv {
    padding: 10px;
    display: flex;
    font-size: 12px;
    font-weight: 600;
    * {
        margin-right: 5px;
        margin-top: 0px;
        margin-bottom: 0px;
        line-height: 35px;
    }
    .status {
        font-size: 14px;
        font-weight: 700;
    }
}

.buttonDiv {
    padding: 10px;
    display: flex;
    flex-wrap: wrap;
    .el-button {
        margin: 7px;
        line-height: 32px;
    }
}

.seek {
    display: flex;
    flex-wrap: wrap;
    flex-direction: column;
    height: 100%;
    .smovList {
        width: 100%;
        flex-grow: 1;
    }
    .zw {
        height: 10px;
        width: 100%;
    }
}
.header {
    padding: 0;
}
.main {
    padding: 5px;
}
</style>





               
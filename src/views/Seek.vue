<template>
  <div
    class="FloatingBallMain"
    :class="pool.isRunning() ? 'onLoad' : ''"
    v-show="!Suspended"
    @dblclick="ChangeSuspended"
    data-tauri-drag-region
  >
    <div
      class="FloatingBall"
      :class="pool.isRunning() ? 'onLoadOut' : ''"
      data-tauri-drag-region
    >
      <!-- <p class="FloatingBallText" data-tauri-drag-region>
        <span class="PoolALL">{{ pool.tasks.length }}</span>
        <span class="Separate">-</span>
        <span class="PoolSuss">{{
            XEUtils.filter(pool.tasks, item => item.status == 1).length
        }}</span>
        <span class="Separate">-</span>
        <span class="PoolFail">
          {{
             XEUtils.filter(pool.tasks, item => item.status == 2).length
          }}</span
        >
      </p> -->
    </div>
  </div>

  <el-container class="seekMain" v-show="Suspended">
    <!-- 检索页面悬浮球 四月份 -->
    <el-header class="header" height="2.1rem">
      <action-bar :imize="false" :minImize="false" :top="true">
        <action-bar-button v-show="pool.delLoading">
          <loading class="onLoad" theme="outline" size="16" fill="#ff2648" />
        </action-bar-button>

        <action-bar-button @click="ChangeSuspended">
          <multilayer-sphere theme="outline" size="16" fill="#333" />
        </action-bar-button>

        <action-bar-button @click="removeAll">
          <delete theme="outline" size="16" fill="#333" />
        </action-bar-button>

        <action-bar-button v-show="!pool.isRunning()" @click="start">
          <find theme="outline" size="16" fill="#333" />
        </action-bar-button>

        <action-bar-button v-show="pool.isRunning()" @click="stop">
          <find theme="outline" size="16" fill="#333" v-show="!pool.time" />

          <find theme="filled" size="16" fill="#333" v-show="pool.time" />
        </action-bar-button>
      </action-bar>
    </el-header>
    <el-main class="main">
      <!-- 版本3 删除会出现卡死现象  方案为 数组队列和虚拟渲染的界面 -->
      <div class="seek">
        <div class="settingDiv">
          <div class="buttonDiv">
            <!-- 图标 方案一  -->
            <!-- <el-icon
                            :size="30"
                            @click="start"
                            class="control"
                            v-show="!pool.isRunning() && !pool.delLoading"
                        >
                            <video-play />
                        </el-icon>

                        <el-icon :size="29" class="control onLoad" v-show="pool.delLoading">
                            <loading />
                        </el-icon>

                        <el-icon
                            :size="30"
                            class="control"
                            v-show="pool.isRunning() && !pool.delLoading"
                        >
                            <video-pause />
                        </el-icon>

                        <el-icon
                            :size="30"
                            class="control"
                            @click="removeAll"
                        >
                            <delete />
                        </el-icon>-->
          </div>

          <div class="filtersDiv">
            <!-- <p>
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
                        </p>-->
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
            :tooltip-config="{ showAll: false, enterDelay: 9999999 }"
          >
            <template #empty>
              <el-empty
                style="line-height: 50px"
                description="没有其他数据了哦"
              ></el-empty>
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
                    :class="
                      row.status == 1
                        ? 'smovCard_suss'
                        : row.status == 2
                        ? 'smovCard_fail'
                        : row.status == 3
                        ? 'smovCard_seeking'
                        : ''
                    "
                  >
                    <div class="smovName">{{ row.seek_name }}</div>
                    <div class="loadingDiv" v-if="row.status == 3">
                      <loading-one
                        theme="outline"
                        size="16"
                        fill="#4a4a4a"
                        class="is-loading onLoad"
                      />
                    </div>

                    <div class="close">
                      <el-button
                        type="danger"
                        size="small"
                        color="#b1b3b8"
                        @click="deleteTask(row)"
                        v-if="row.status != 3"
                        :icon="DeleteFilled"
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
              :filters="[
                { label: '错误', value: 2, checked: openStatus[2] },
                { label: '等待', value: 0, checked: openStatus[0] },
                { label: '成功', value: 1, checked: openStatus[1] },
                { label: '正在执行', value: 3, checked: openStatus[3] },
              ]"
            ></vxe-column>
          </vxe-table>
        </div>

        <div class="zw"></div>
      </div>
    </el-main>
  </el-container>
</template>

<script lang='ts' setup>
import {
  defineComponent,
  ref,
  reactive,
  inject,
  watch,
  getCurrentScope,
  onMounted,
  onUpdated,
  nextTick,
} from "vue";
import { ThreadPool } from "../ts/ThreadPool";
import { invoke } from "@tauri-apps/api/tauri";
import { getAll, getCurrent } from "@tauri-apps/api/window";
import { listen } from "@tauri-apps/api/event";
import { DeleteFilled } from "@element-plus/icons-vue";
import { ElMessage, ElLoading } from "element-plus";
import "element-plus/es/components/message/style/css";
import "element-plus/es/components/loading/style/css";
import XEUtils from "xe-utils";
import {
  VXETable,
  VxeTableInstance,
  VxeTableEvents,
  RecordInfo,
  VxeColumnPropTypes,
} from "vxe-table";
import {
  PlayOne,
  Pause,
  Loading,
  Delete,
  Find,
  LoadingOne,
  MultilayerSphere,
} from "@icon-park/vue-next";
import { request } from "../util/invoke";

const Tasks = ref({} as VxeTableInstance);

const HotLoading = ref(false);

const Suspended = ref(true);

const openStatus = ref([
  true, //wait
  true, //suss
  true, //fail
  true, //run time
  true, //delete run time
]);

let pool = reactive(
  new ThreadPool.FixedThreadPool({
    size: 1,
    runningFlag: false,
    autoRun: false,
  })
);

onUpdated(() => {});

onBeforeUpdate(() => {});

const filterStatusMethod: VxeColumnPropTypes.FilterMethod = ({
  value,
  row,
}) => {
  console.log(value);
  return openStatus.value[row.status];
};

const addTaskEvent = () => {
  !(async () =>
    await listen("addTask", (event: any) => {
      HotLoading.value = true;
      console.log(Date.now());
      asyncJoin(event.payload);
    }))();
};

const asyncJoin = async (list: any[]) => {
  pool.addTasks(list);
  Tasks.value.reloadData(pool.tasks).then(() => {
    setTimeout(() => {
      HotLoading.value = false;
    }, 200);
  });

  console.log(pool.tasks.length);
  console.log(Tasks.value.getData().length);
};

const getSeekSmov = () => {
  pool.loading = true;
  pool.tasks = [];
  Tasks.value.remove();
  invoke("get_seek_smov")
    .then((res: any) => {
      let data: any = res;
      if (data.code == 200) {
        pool.addTasks(data.data);
        //获取最后的索引下标
        let index = XEUtils.findIndexOf(data.data, (item) => item.status === 0);
        if (index == -1) {
          pool.index = 0;
        } else {
          pool.index = index;
        }
      }
    })
    .finally(() => {
      const $table = Tasks.value;
      if ($table) {
        const data = pool.tasks;
        $table.loadData(data);
      }
      pool.loading = false;
    });
};

onMounted(() => {
  nextTick(() => {
    getSeekSmov();
  });
  addTaskEvent();
});

const getFilter = () => {
  return [
    { label: "等待", value: 0, checked: openStatus.value[0] },
    { label: "成功", value: 1, checked: openStatus.value[1] },
    { label: "错误", value: 2, checked: openStatus.value[2] },
    { label: "正在执行", value: 3, checked: openStatus.value[3] },
  ];
};

const ErrChange = (val: any) => {
  const $table = Tasks.value;
  openStatus.value[2] = val;
  const column = $table.getColumnByField("status");
  if (column) {
    const filter = getFilter();
    $table.setFilter(column, filter);
    $table.updateData();
  }
};

const SussChange = (val: any) => {
  const $table = Tasks.value;
  openStatus.value[1] = val;
  const column = $table.getColumnByField("status");
  if (column) {
    const filter = getFilter();
    $table.setFilter(column, filter);
    $table.updateData();
  }
};

const AlwaysOnTop: any = inject("AlwaysOnTop");

const ChangeSuspended = () => {
  request("change_seek_suspended", { flag: Suspended.value }).then(() => {
    Suspended.value = !Suspended.value;

    // nextTick(() => {
    //   getCurrent().show();
    // });

    

    setTimeout(() => {
      getCurrent().show();
    }, 50);
    //重置窗口是否置顶
    if (Suspended.value) {
      getCurrent().setAlwaysOnTop(AlwaysOnTop.value);
    }
  });
};

const WaitChange = (val: any) => {
  const $table = Tasks.value;
  openStatus.value[0] = val;
  const column = $table.getColumnByField("status");
  if (column) {
    const filter = getFilter();
    $table.setFilter(column, filter);
    $table.updateData();
  }
};

const removeAll = () => {
  pool.loading = true;

  const data: number[] = XEUtils.map(pool.tasks, (item) => item.id);

  invoke("remove_smov_seek_status", { id: data })
    .then((res: any) => {
      if (res.code == 200) {
        Tasks.value.remove();
        pool.tasks = [];
        pool.index = 0;
      } else {
        ElMessage.error("移除检索队列出现错误");
        return;
      }
    })
    .finally(() => {
      pool.loading = false;
    });
};

const start = () => {
  pool.start();
};

const stop = () => {
  pool.stop();
};

const close = () => {
  getCurrent().hide();
};

const deleteTask = (row: { status: number; id: any }) => {
  row.status = 3;
  invoke("remove_smov_seek_status", { id: [row.id] }).then((res: any) => {
    if (res.code == 200) {
      const $table = Tasks.value;
      let index = $table.getRowIndex(row);
      $table.remove(row);
      XEUtils.remove(pool.tasks, (item) => item.id === row.id);
      if (index + 1 <= pool.index) {
        pool.index--;
      }
    } else {
      ElMessage.error("移除检索队列出现错误");
      return;
    }
  });
};
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
  padding-top: 9px;
  padding-bottom: 9px;
  width: 100%;
  height: 100%;
  display: flex;
  justify-content: center;
}

.smovCard {
  width: 96%;
  height: 40px;
  line-height: 40px;
  border-radius: 5px;
  box-shadow: 0px 0px 9px rgba(0, 0, 0, 0.12);
  font-weight: 600;
  padding: 1px;
  .el-card__body {
    padding: 0;
    font-size: 1rem;
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
  // width: 100%;
  // height: 100%;
  border-radius: 50%;
  position: absolute;
  top: 5%;
  left: 2%;
  // background: #ffffffe6;
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
  right: 8%;
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
  // padding: 10px;
  display: flex;
  font-size: 0.8rem;
  font-weight: 600;
  * {
    margin-right: 5px;
    margin-top: 0px;
    margin-bottom: 0px;
    line-height: 35px;
  }
}

.status {
  font-size: 1rem;
  font-weight: 700;
  margin-right: 20px;
}

.buttonDiv {
  display: flex;
  flex-wrap: wrap;
  justify-content: left;
  .el-button {
    margin: 7px;
    line-height: 32px;
  }
  .control {
    border-radius: 50%;
    margin-left: 20px;
    cursor: pointer;
  }
  .control:hover {
    background-color: rgba(0, 0, 0, 0);
  }
}

.seek {
  display: flex;
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

@keyframes rotating {
  from {
    transform: rotate(0);
  }
  to {
    transform: rotate(360deg);
  }
}

@keyframes rotatingOut {
  from {
    transform: rotate(360deg);
  }
  to {
    transform: rotate(0);
  }
}

.onLoad {
  animation: rotating 3s linear infinite;
}

.onLoadOut {
  animation: rotatingOut 3s linear infinite;
}

.FloatingBall {
  background-color: #edb72c;
  width: 80%;
  height: 80%;
  border-radius: 50%;
  box-shadow: 0 0 8px rgba(0, 0, 0, 0.4);
  display: flex;
  justify-content: center;
  align-items: center;
}
.FloatingBallMain {
  width: 100vw;
  height: 100vh;
  display: flex;
  justify-content: center;
  align-items: center;
  margin: 0;
  padding: 0;
  background-color: #c7415b;
  cursor: all-scroll;
  background-image: linear-gradient(
      45deg,
      #edb72c 50%,
      transparent 0,
      transparent 100%,
      #edb72c 0
    ),
    linear-gradient(
      45deg,
      #edb72c 50%,
      transparent 0,
      transparent 100%,
      #edb72c 0
    );

  border-radius: 50%;
}

.bg-liuguang {
  animation: liuguang 2s infinite linear;
}

#app {
  overflow: hidden;
}

@keyframes liuguang {
  0% {
    background: linear-gradient(
      to bottom right,
      #8a8af4 0%,
      #ffa0ec 100%,
      #8a8af4 200%,
      #ffa0ec 300%
    );
  }

  5% {
    background: linear-gradient(
      to bottom right,
      #8a8af4 -10%,
      #ffa0ec 90%,
      #8a8af4 190%,
      #ffa0ec 290%
    );
  }

  10% {
    background: linear-gradient(
      to bottom right,
      #8a8af4 -20%,
      #ffa0ec 80%,
      #8a8af4 180%,
      #ffa0ec 280%
    );
  }

  15% {
    background: linear-gradient(
      to bottom right,
      #8a8af4 -30%,
      #ffa0ec 70%,
      #8a8af4 170%,
      #ffa0ec 270%
    );
  }

  20% {
    background: linear-gradient(
      to bottom right,
      #8a8af4 -40%,
      #ffa0ec 60%,
      #8a8af4 160%,
      #ffa0ec 260%
    );
  }

  25% {
    background: linear-gradient(
      to bottom right,
      #8a8af4 -50%,
      #ffa0ec 50%,
      #8a8af4 150%,
      #ffa0ec 250%
    );
  }

  30% {
    background: linear-gradient(
      to bottom right,
      #8a8af4 -60%,
      #ffa0ec 40%,
      #8a8af4 140%,
      #ffa0ec 240%
    );
  }

  35% {
    background: linear-gradient(
      to bottom right,
      #8a8af4 -70%,
      #ffa0ec 30%,
      #8a8af4 130%,
      #ffa0ec 230%
    );
  }

  40% {
    background: linear-gradient(
      to bottom right,
      #8a8af4 -80%,
      #ffa0ec 20%,
      #8a8af4 120%,
      #ffa0ec 220%
    );
  }

  45% {
    background: linear-gradient(
      to bottom right,
      #8a8af4 -90%,
      #ffa0ec 10%,
      #8a8af4 110%,
      #ffa0ec 210%
    );
  }

  50% {
    background: linear-gradient(
      to bottom right,
      #8a8af4 -100%,
      #ffa0ec 0%,
      #8a8af4 100%,
      #ffa0ec 200%
    );
  }

  55% {
    background: linear-gradient(
      to bottom right,
      #8a8af4 -110%,
      #ffa0ec -10%,
      #8a8af4 90%,
      #ffa0ec 190%
    );
  }

  60% {
    background: linear-gradient(
      to bottom right,
      #8a8af4 -120%,
      #ffa0ec -20%,
      #8a8af4 80%,
      #ffa0ec 180%
    );
  }

  65% {
    background: linear-gradient(
      to bottom right,
      #8a8af4 -130%,
      #ffa0ec -30%,
      #8a8af4 70%,
      #ffa0ec 170%
    );
  }

  70% {
    background: linear-gradient(
      to bottom right,
      #8a8af4 -140%,
      #ffa0ec -40%,
      #8a8af4 60%,
      #ffa0ec 160%
    );
  }

  75% {
    background: linear-gradient(
      to bottom right,
      #8a8af4 -150%,
      #ffa0ec -50%,
      #8a8af4 50%,
      #ffa0ec 150%
    );
  }

  80% {
    background: linear-gradient(
      to bottom right,
      #8a8af4 -160%,
      #ffa0ec -60%,
      #8a8af4 40%,
      #ffa0ec 140%
    );
  }

  85% {
    background: linear-gradient(
      to bottom right,
      #8a8af4 -170%,
      #ffa0ec -70%,
      #8a8af4 30%,
      #ffa0ec 130%
    );
  }

  90% {
    background: linear-gradient(
      to bottom right,
      #8a8af4 -180%,
      #ffa0ec -80%,
      #8a8af4 20%,
      #ffa0ec 120%
    );
  }

  95% {
    background: linear-gradient(
      to bottom right,
      #8a8af4 -190%,
      #ffa0ec -90%,
      #8a8af4 10%,
      #ffa0ec 110%
    );
  }

  100% {
    background: linear-gradient(
      to bottom right,
      #8a8af4 -200%,
      #ffa0ec -100%,
      #8a8af4 0%,
      #ffa0ec 100%
    );
  }
}

.FloatingBallText {
  font-size: 0.9rem;
  font-weight: 700;
}

.Separate {
  color: black;
}

.PoolSuss {
  color: black;
}

.PoolFail {
  color: black;
}
</style>

<style lang="less">
.vxe-table--body-wrapper {
  overflow: overlay;
}

.seekMain {
  background-color: white;
}
</style>





               
<template>
  <div class="log" :class="isOpen ? 'log_open' : 'log_close'">
    <button class="Btn" @click="logClick" :class="isOpen ? 'Btn_open' : 'Btn_close'">{{ isOpen }}</button>
    <div class="logList" :class="isOpen ? 'logList_open' : 'logList_close'">
      <div class="logDiv" id="logDiv">
        <div class="logItem" v-for="(item, index) in logList" :key="index">
          <span class="time">{{ item.time }}</span>
          <span class="ON">ON</span>
          <span class="time">{{ item.thread }}:</span>
          <span class="level" :class="item.level">{{ item.level }}</span>
          <span class="msg" :class="item.level">{{ item.msg }}</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref, onMounted, inject, watch, computed, reactive, onUpdated, nextTick } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from '@tauri-apps/api/event';
import { Log } from '@/type/log'
import { CurentTime } from '@/util/time'

//我需要一个seek页面的监听器 当传入的数量大于一定值时 我需要他转起来

export default defineComponent({
  components: {},
  name: "Log",
  setup() {   
    const isOpen = ref(false);
    const logList = ref([] as any[]);
    const log = inject('log') as any;
    watch(logList.value, () => {
      if (logList.value.length > 50) {
        logList.value.shift()
      }
    });

    watch(log, () => {
       logList.value.push(log.value);
    })

    onUpdated(() => {
      scrollToBottom();
    })

    const scrollToBottom = () => {
      nextTick(() => {
        let logDiv = document.getElementById('logDiv');
        if (logDiv != null) {
          logDiv.scrollTop = logDiv.scrollHeight;
        }
      }
      )
    }
    const logClick = () => {
      isOpen.value = !isOpen.value;
      let logDiv = document.getElementById('logDiv');
      if (logDiv != null) {
        logDiv.focus();
      }
    }

    //前端log可视 存储用localstorage 用provide 和 inject传递给log组件
    onMounted(() => {
      eventLog();
    });

    const eventLog = () => {
      !(async () => await listen('frontend_log', (event: any) => {
        const log: Log = {
          time: CurentTime(),
          thread: event.payload.thread,
          msg: event.payload.fields.message,
          level: event.payload.level
        };
        logList.value.push(log);
      }))()
    }

    return {
      logList,
      isOpen,
      logClick
    };
  },
});
</script>

<style lang="less" scoped>
@log_background: #f1f1f1;
@log_shadow_close: 0 -2px 3px -1px rgb(133, 133, 133);
@log_shadow_open: 0 -2px 3px -1px rgb(136, 0, 0);
.log {
  position: fixed;
  bottom: 0;
  left: 0;
  z-index: 100;
  width: 100%;
  background: @log_background;
  box-shadow: @log_shadow_close; // 顶部阴影
  font-size: 1rem;
}

.logDiv {
  position: absolute;
  bottom: 0px;
  overflow: auto;
  height: 100%;
  width: 100%;
}
.log_open {
  height: 16em;
}

.log_close {
  height: 3em;
}
.logList {
  display: flex;
  flex-direction: column;
  position: relative;
  top: 0px;
  height: 100%;
}

.logList_open {
  overflow: auto;
}

.logList_close {
  overflow: hidden;
  ::-webkit-scrollbar {
    display: none; /* Chrome Safari */
  }
}

.Btn {
  position: absolute;
  right: 0;
  bottom: 100%;
  border: 0px;
  background: @log_background;
  box-shadow: @log_shadow_close;
  height: 20px;
  width: 50px;
  border-radius: 5px 0 0 0;
  font-weight: 700;
}

.Btn_open {
  box-shadow: @log_shadow_open;
}

.Btn_close {
  box-shadow: @log_shadow_close;
}

.logItem {
  display: flex;
  align-items: center;
  margin-left: 10px;
  flex-wrap: wrap;
  margin-top: 0.5rem;
  height: 2.5rem;
}
.level {
  font-weight: 700;
  margin-right: 7px;
}

.INFO {
  color: rgb(0, 165, 82);
}
.TRACE {
  color: rgb(94, 94, 94);
}
.DEBUG {
  color: rgb(34, 34, 34);
}

.WARN {
  color: rgb(160, 149, 0);
}

.ERROR {
  color: rgb(231, 0, 0);
}
.time {
  font-weight: 800;
  margin-right: 5px;
}
.msg {
  font-weight: 600;
}

.ON {
  font-weight: 800;
  color: rgb(160, 160, 160);
  margin-right: 5px;
}
</style>
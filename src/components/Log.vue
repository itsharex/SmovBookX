<template>
  <div class="log" :class="isOpen ? 'log_open' : 'log_close'">
    <button class="Btn" @click="logClick" :class="isOpen ? 'Btn_open' : 'Btn_close'">{{ isOpen }}</button>
    <div class="logList">
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
import { defineComponent, ref, onMounted, inject, watch, computed, reactive } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from '@tauri-apps/api/event';

export default defineComponent({
  components: {},
  name: "Log",
  setup() {

    const isOpen = ref(false);

    const logList = ref([] as any[]);

    watch(logList.value, () => {
      if (logList.value.length > 50) {
        logList.value.shift()
      }
      // console.log(res)
    });

    const logClick = () => {
      isOpen.value = !isOpen.value;
      let logDiv = document.getElementById('logDiv');
      if (logDiv != null) {
        logDiv.scrollTop = logDiv.scrollHeight;
      }
    }

    //前端log可视 存储用localstorage 用provide 和 inject传递给log组件
    onMounted(() => {
      eventLog()
    });

    const eventLog = () => {
      !(async () => await listen('frontend_log', (event: any) => {
        const log = {
          time: CurentTime(),
          thread: event.payload.thread,
          msg: event.payload.fields.message,
          level: event.payload.level
        };
        logList.value.push(log);
        let logDiv = document.getElementById('logDiv');
        if (logDiv != null) {
          logDiv.scrollTop = logDiv.scrollHeight;
        }
      }))()
    }

    function CurentTime() {
      var now = new Date();

      var year = now.getFullYear();
      var month = now.getMonth() + 1;
      var day = now.getDate();
      var hh = now.getHours();
      var mm = now.getMinutes();
      var ss = now.getSeconds();
      var clock = year + "-";
      if (month < 10) clock += "0";
      clock += month + "-";
      if (day < 10) clock += "0";
      clock += day + " ";
      if (hh < 10) clock += "0";
      clock += hh + ":";
      if (mm < 10) clock += '0';
      clock += mm + ":";
      if (ss < 10) clock += '0';
      clock += ss;
      return (clock);
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
@log_shadow: 0 -2px 3px -1px rgb(133, 133, 133);
.log {
  position: fixed;
  bottom: 0;
  left: 0;
  z-index: 100;
  width: 100%;
  background: @log_background;
  box-shadow: @log_shadow; // 顶部阴影
  font-size: small;
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
  height: 2em;
}
.logList {
  display: flex;
  flex-direction: column;
  // align-items: flex-end;
  position: relative;
  top: 0px;
  overflow: auto;
  height: 100%;
}

.Btn {
  position: absolute;
  right: 0;
  bottom: 100%;
  border: 0px;
  background: @log_background;
  box-shadow: @log_shadow;
  height: 20px;
  width: 50px;
  border-radius: 5px 0 0 0;
  font-weight: 700;
}

.Btn_open {
}

.Btn_close {
}

.logItem {
  // height: 2em;
  display: flex;
  align-items: center;
  margin-left: 10px;
  flex-wrap:wrap;
  margin-bottom: 12px;
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
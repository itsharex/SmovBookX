<template>
  <div class="log" :class="isOpen ? 'log_open' : 'log_close'">
    <div class="logList">
      <div class="logItem" v-for="(item, index) in logList" :key="index">
        <span>{{ item.time }}:</span>
        <span>{{ item.msg }}</span>
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
      if (logList.value.length > 500) {
        logList.value.shift()
      }
      // console.log(res)
    });

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
        logList.value.push(log)
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
      isOpen
    };
  },
});
</script>

<style lang="less" scoped>
.log {
  position: fixed;
  bottom: 0;
  z-index: 100;
}
.log_open {
  height: 5em;
}

.log_close {
  height: 100em;
}
.logList {
  display: flex;
  flex-direction:column;
}

.logItem {
  height: 2em;
}
</style>
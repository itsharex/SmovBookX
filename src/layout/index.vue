<template>
  <router-view></router-view>
</template>

<script setup lang="ts">
import {
  defineComponent,
  ref,
  onMounted,
  provide,
  watch,
  computed,
  reactive,
  nextTick,
} from "vue";

const logs = ref("" as any);
const seek = ref(false);
//定义一个全局值 ，可以由前端打印日志
provide("log", logs);

watch(seek, () => {
  console.log(seek.value);
});

//检索状态全局值
provide("seek", seek);

onMounted(() => {
  disableMenu();
});

const disableMenu = () => {
  if (window.location.hostname !== "tauri.localhost") {
    return;
  }

  document.addEventListener(
    "contextmenu",
    (e) => {
      e.preventDefault();
      return false;
    },
    { capture: true }
  );

  document.addEventListener(
    "selectstart",
    (e) => {
      e.preventDefault();
      return false;
    },
    { capture: true }
  );
};
</script>



<style lang="less">
html,
body {
  background-color: rgba(255, 255, 255, 0);
  border-radius: 7px;
  font-size: 15px;
}
</style>


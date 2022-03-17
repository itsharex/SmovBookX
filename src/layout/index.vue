<template>
  <router-view></router-view>
</template>

<script setup lang="ts">
import { defineComponent, ref, onMounted, provide, watch, computed, reactive, nextTick } from "vue";
import { checkUpdate, installUpdate } from "@tauri-apps/api/updater";
import { relaunch } from "@tauri-apps/api/process";

const logs = ref('' as any);
const seek = ref(false);
//定义一个全局值 ，可以由前端打印日志
provide('log', logs);

watch(seek, () => {
  console.log(seek.value);
})

//检索状态全局值
provide('seek', seek);

onMounted(() => {
  test();
  disableMenu();
  // nextTick(() => {
  //   disableMenu();

  // })
});

const disableMenu = () => {
  if (window.location.hostname !== 'tauri.localhost') {
    return
  }

  document.addEventListener('contextmenu', e => {
    e.preventDefault();
    return false;
  }, { capture: true })

  document.addEventListener('selectstart', e => {
    e.preventDefault();
    return false;
  }, { capture: true })
}

const test = async () => {
  try {
    console.log("检查版本更新")
    const { shouldUpdate, manifest } = await checkUpdate();
    console.log(shouldUpdate)
    if (shouldUpdate) {
      // display dialog
      await installUpdate();
      // install complete, restart app
      await relaunch();
    }
  } catch (error) {
    console.log(error);
  }
}

</script>



<style lang="less">
body {
  background-color: #00000000;
}
</style>

<template>
  <router-view></router-view>
</template>

<script setup lang="ts">
import { defineComponent, ref, onMounted, provide, watch, computed, reactive } from "vue";
const logs = ref('' as any);
const seek = ref(false);
//定义一个全局值 ，可以由前端打印日志
provide('log', logs);

watch(seek, () => {
  console.log(seek.value);
})

//检索状态全局值
provide('seek', seek);

</script>

<style lang="less">
</style>

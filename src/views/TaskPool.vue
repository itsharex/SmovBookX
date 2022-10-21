<template>
  <el-container class="TaskPool">
    <el-header class="header" height="2.1rem">
      <action-bar :imize="false" :minImize="false" :top="true" />
    </el-header>
    <el-container>
      <el-aside class="TaskPoolAside" width="25%">
        <el-input v-model="searchContent"></el-input>
      </el-aside>
      <el-main class="TaskPoolMain"> Main </el-main>
    </el-container>
  </el-container>
</template>

<script lang="ts" setup>
const searchContent = ref("");


//搜索内容防抖 在修改搜索内容时 如果前面还在执行就先清除后继续
let SearchTimeOut: any;

watch(searchContent, (newValue, oldValue) => {
  clearTimeout(SearchTimeOut);
  SearchTimeOut = setTimeout(() => {
    console.log(newValue);
  }, 2000);
});


</script>

<style lang="less" scoped>
.header {
  padding: 0;
}
.TaskPool {
  background: rgb(154, 150, 150);
  width: 100vw;
  height: 100vh;
}
.TaskPoolAside {
  background: white;
  height: 100%;
}
</style>
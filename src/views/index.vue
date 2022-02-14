<template>
  <el-button type="danger" @click="goBack">查询未检索信息</el-button>
  <el-button type="danger" @click="toInit">检索文件系统</el-button>
  <el-button type="danger" @click="toSeek">跳转至正常检索页面</el-button>
  <el-button type="danger" @click="router.push({
        path: '/test',
      });">跳转至测试</el-button>
    <el-button type="danger" @click="router.push({
        path: '/setting',
      });">跳转至设置</el-button>
</template>

<script lang="ts">
import { defineComponent, ref, onMounted, inject, watch, computed } from "vue";
import { useRouter } from "vue-router";
import { ElMessage } from "element-plus";
import { invoke } from "@tauri-apps/api/tauri";

export default defineComponent({
  components: {},
  name: "index",
  setup() {
    const goBack = () => {
      // ElMessage.error("Test.");
      invoke("query_unretrieved").then(res => {
        console.log(res);
      });

    };
    const router = useRouter();
    const toInit = () => {
      // router.push({
      //   path: "/SomvFile",
      // });
      invoke("query_new_file_todb").then(res => {
        console.log(res);
      });
    };

    const toSeek = () => {
      router.push({
        path: "/SomvFile",
      });
    };
    return {
      goBack,
      toInit,
      router,
      toSeek
    };
  }
})

</script>

<style scoped>
</style>
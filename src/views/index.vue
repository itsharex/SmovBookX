<template>
  <!-- <el-button type="danger" @click="goBack">查询未检索信息</el-button> -->
  <el-button type="danger" @click="toInit" :loading="loading">检索文件系统</el-button>
  <el-button type="danger" @click="toSeek">跳转至正常检索页面</el-button>
  <el-button type="danger" @click="
    router.push({
      path: '/test',
    })
  ">跳转至测试</el-button>
  <el-button
    type="danger"
    @click="
      router.push({
        path: '/setting',
      })
    "
  >跳转至设置</el-button>
  <!-- <el-button type="danger" @click="
    router.push({
      path: '/seek',
    })
  ">跳转至检索列表</el-button> -->
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
      invoke("query_unretrieved").then((res) => {
        console.log(res);
      });
    };
    const router = useRouter();
    const loading = ref(false);
    const toInit = () => {
      // router.push({
      //   path: "/SomvFile",
      // });
      loading.value = true;
      invoke("query_new_file_todb")
        .then((res) => {
          const data: any = res;
          if (data.code != 200) {
            ElMessage({
              showClose: true,
              message: "出现了一个错误！",
              type: "error",
            });
          }
        })
        .catch(() => {
          ElMessage({
            showClose: true,
            message: "出现了一个错误！",
            type: "error",
          });
        })
        .finally(() => {
          ElMessage({
            showClose: true,
            message: "检索成功",
            type: "success",
          });
          loading.value = false;
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
      toSeek,
      loading,
    };
  },
});
</script>

<style scoped>
</style>
<template>
  <el-button type="danger" @click="setSeekFolder">添加检索文件夹</el-button>
  <el-button type="danger" @click="setTidyFolder">设置整理文件夹</el-button>
</template>

<script>
import { defineComponent, ref, onMounted, inject, watch, computed } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

export default defineComponent({
  components: {},
  name: "Setting",
  setup() {
    const setSeekFolder = () => {
      invoke("open_folder_select").then((res) => {
        if (res.code == 200) {
          invoke("insert_folder", { path: res.data }).then((res1) => {
            console.log(res1);
          });
        }
      });
    };

    const setTidyFolder = () => {
      // invoke("update_config");
      invoke("test");
    };

    return {
      setSeekFolder,
      setTidyFolder
    };
  },
});
</script>
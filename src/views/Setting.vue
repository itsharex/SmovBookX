<template>
  <el-button type="danger" @click="setSeekFolder">添加检索文件夹</el-button>
  <el-button type="danger" @click="setTidyFolder">设置整理文件夹</el-button>
  <el-button type="danger" @click="test">ces</el-button>
</template>

<script lang="ts">
import { defineComponent, ref, onMounted, inject, watch, computed } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { dialog } from "@tauri-apps/api";
import { OpenDialogOptions } from "@tauri-apps/api/dialog";

export default defineComponent({
  components: {},
  name: "Setting",
  setup() {
    const setSeekFolder = () => {
      invoke("open_folder_select").then((res: any) => {
        if (res.code == 200) {
          invoke("insert_folder", { path: res.data }).then((res1: any) => {
            console.log(res1);
          });
        }
      });
    };

    const setTidyFolder = () => {
      invoke("open_folder_select").then((res: any) => {
        if (res.code == 200) {
          invoke("update_tidy_folder", { path: res.data });
        }
      });
    };
    const test = () => {
      invoke("test");
      // let option: OpenDialogOptions = {
      //   directory: true
      // }
      // dialog.open(option).then((res: any) => {
      //   console.log(res)
      // })

    };

    return {
      setSeekFolder,
      setTidyFolder,
      test
    };
  },
});
</script>
<template>
<el-button type="danger" @click="getPath">跳转至正常检索页面</el-button>
</template>

<script>
import { defineComponent, ref, onMounted, inject, watch, computed } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

export default defineComponent({
  components: {},
  name: "Setting",
  setup() {
    const getPath = () => {
       invoke("open_folder_select").then(res =>{
         if (res.code == 200){
           invoke("insert_folder",{path:res.data}).then(
             res1 =>{
               console.log(res1)
             }
           )
         }
       } );
    };
    return {
      getPath,
    };
  },
});
</script>
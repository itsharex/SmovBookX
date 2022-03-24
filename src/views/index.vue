<template>
  <div>
    <el-button type="info" color = "#C7415B" @click="toInit" :loading="loading">检索文件系统</el-button>
    <el-button type="info" color = "#C7415B" @click="toSeek">跳转至正常检索页面</el-button>
    <el-button
      type="info"
      color = "#C7415B"
      @click="
        router.push({
          path: '/test',
        })
      "
    >跳转至测试</el-button>

    <el-button
      type="info"
      color = "#C7415B"
      @click="
        router.push({
          path: '/SomvView',
        })
      "
    >跳转至灵魂</el-button>

    <el-button
      type="info"
      color = "#C7415B"
      @click="
        router.push({
          path: '/setting',
        })
      "
    >跳转至设置</el-button>

    <p>温馨提醒 ： 已经加了图片功能，谨防社死</p>

    <!-- 检索提示 -->
    <el-dialog v-model="Dialog.show" title="回调" width="50%" destroy-on-close center>
      <p>
        检索到
        <span class="number">{{ Dialog.data.add_size }}</span>
        条新数据
      </p>
      <p v-if="Dialog.data.del_smov_file.length != 0">
        发现
        <span class="number">{{ Dialog.data.del_smov_file.length }}</span> 条被删除的数据
      </p>
      <p>
        <el-checkbox
          label="删除已被删除的数据"
          v-model="Dialog.del"
          v-if="Dialog.data.del_smov_file.length != 0"
        />
      </p>
      <template #footer>
        <span class="dialog-footer">
          <el-button type="primary" @click="DialogClick">确定</el-button>
        </span>
      </template>
    </el-dialog>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref, onMounted, inject, watch, computed, render, h } from "vue";
import { useRouter } from "vue-router";
import { ElMessage } from "element-plus";
import { invoke } from "@tauri-apps/api/tauri";
import XEUtils from "xe-utils";
import 'element-plus/es/components/message/style/css'

export default defineComponent({
  components: {},
  name: "index",
  setup() {

    const goBack = () => {
      invoke("query_unretrieved").then((res) => {
        console.log(res);
      });
    };

    const Dialog = ref({
      show: false,
      loading: false,
      data: {} as any,
      del: true
    });

    const router = useRouter();

    const loading = ref(false);

    const toInit = () => {
      loading.value = true;
      invoke("query_new_file_todb")
        .then((res) => {
          const data: any = res;
          console.log(data)
          if (data.code == 200) {
            Dialog.value.data = data.data;
            Dialog.value.show = true;
          } else {
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
          setTimeout(() => {
            loading.value = false;
          }, 500);

        });
    };

    const toSeek = () => {
      router.push({
        path: "/SomvFile",
      });
    };

    const DialogClick = () => {
      Dialog.value.loading = true;
      if (Dialog.value.del && Dialog.value.data.del_smov_file.length != 0) {
        const data: number[] = XEUtils.map(Dialog.value.data.del_smov_file as any[], item => item.id);
        invoke("delete_smov", { id: data }).then((res: any) => {
          if (res.code = 200) {
            ElMessage({
              showClose: true,
              message: "检索成功,删除了" + Dialog.value.data.del_smov_file.length + "条数据",
              type: "success",
            });
          }

        }
        )
      } else {
        ElMessage({
          message: "检索成功!",
          type: "success",
        });
      };
      Dialog.value.show = false;
    }

    return {
      goBack,
      toInit,
      router,
      toSeek,
      loading,
      Dialog,
      DialogClick
    };
  },
});
</script>

<style scoped>
.dialog {
  line-height: 20px;
}

.number {
  font-weight: 600;
}
</style>
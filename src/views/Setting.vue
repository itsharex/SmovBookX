<template>
  <!-- 设置界面优化 本周 -->
  <div class="setting">
    <p align="left" class="title">设置</p>
    <div class="settingItem tidy">
      <span>整理文件夹:</span>
      <file-input
        ref="tidy"
        :directory="true"
        @check="setTidyFolder"
        :value="conf.conf.tidy_folder"
      />
    </div>

    <div class="settingItem thread" v-show="false">
      <span>检索线程数:</span>
      <file-input :directory="true" @check="setSeekFolder" />
    </div>

    <div class="settingItem seek">
      <el-popover placement="bottom" :width="400" trigger="click">
        <template #reference>
          <span>检索文件夹:</span>
        </template>
        <!-- el的table貌似有个滑动监听 有war -->
        <el-table height="200" :data="conf.seek_folder">
          <el-table-column label="path">
            <template #default="scope">
              <div class="PathColumn">
                <div class="PathColumnText">
                  <span>{{ scope.row.path }}</span>
                </div>
                <div class="PathClose">
                  <el-icon :size="18" @click="deleteSeekFolder(scope.row.id)">
                    <close />
                  </el-icon>
                </div>
              </div>
            </template>
          </el-table-column>
        </el-table>
      </el-popover>

      <file-input ref="seek" :directory="true" @check="setSeekFolder" />
    </div>
  </div>
</template>

<script lang="ts" setup>
import { defineComponent, ref, onMounted, inject, watch, computed } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { dialog } from "@tauri-apps/api";
import { OpenDialogOptions } from "@tauri-apps/api/dialog";
import { request } from "../util/invoke";
import { Close } from '@element-plus/icons-vue';
import XEUtils from "xe-utils";

const { $alert } = getCurrentInstance()!.appContext.config.globalProperties;

const conf = ref({
  conf: {
    thread: 0,
    tidy_folder: ""
  },
  seek_folder: [

  ]
} as any);

const tidy = ref();
const seek = ref();

const setSeekFolder = (path: any) => {
  request("insert_folder", { path: path }).then((callback: any) => {
    if (callback.code == 200) {
      $alert.success("添加成功")
      conf.value.seek_folder.push({
        id:callback.data,
        path:path
      });
      seek.value.clean();
    }
  });
};

const initSettingData = () => {
  request("get_setting_data").then((res: any) => {
    console.log(res);
    conf.value = res.data;
  })
}

const setTidyFolder = (res: any) => {
  request("update_tidy_folder", { path: res }).then(() => {
    conf.value.conf.tidy_folder = res;
    tidy.value.clean();
  }

  );
};

const deleteSeekFolder = (id: any) => {
  request("delete_folder", { id: id }).then((res: any) => {
    $alert.success("删除成功")
    XEUtils.remove(conf.value.seek_folder, item => item.id === id)
  })
}

onMounted(() => {
  initSettingData();
})

</script>

<style lang="less" scoped>
.title {
  font-size: 30px;
  font-weight: 700;
  margin: 0;
  padding: 12px;
  margin-bottom: 2rem;
}

.setting {
  display: flex;
  flex-wrap: wrap;
  flex-direction: column;
  justify-content: left;
  div {
    width: 80%;
    padding: 4px;
  }
}

.settingItem {
  display: flex;
  flex-direction: row;
  flex-wrap: nowrap;
  justify-content: left;
  align-items: center;
  span {
    width: 30%;
    font-weight: 600;
  }
}

.seek {
  span {
    cursor: pointer;
  }
}

.PathColumn {
  display: flex;
  flex-direction: row;
  align-items: center;
  .PathColumnText {
    height: 100%;
  }
  .PathClose {
    height: 100%;
    flex-grow: 1;
    display: flex;
    justify-content: right;
    align-items: center;
    width: 20px;
    .el-icon {
      margin-right: 7px;
      cursor: pointer;
    }
  }
}
</style>
<template>
  <div class="fileMain">
    <vxe-toolbar export :refresh="{ query: findList }">
      <template #buttons>
        <vxe-button @click="getSelectEvent">获取选中</vxe-button>
        <vxe-input v-model="search" type="search" placeholder="试试全表搜索" @keyup="searchEvent"></vxe-input>
      </template>
    </vxe-toolbar>

    <vxe-table
      resizable
      show-overflow
      keep-source
      ref="xTable"
      class="fileTable"
      :export-config="{}"
      :loading="table.loading"
      :checkbox-config="{ checkField: 'checked' }"
      :edit-config="{ trigger: 'click', mode: 'row', showStatus: true }"
      @edit-closed="editClosedEvent"
    >
      <vxe-column type="seq" width="100"></vxe-column>
      <vxe-column type="checkbox" width="60"></vxe-column>
      <vxe-column field="realname" title="文件名称"></vxe-column>
      <vxe-column field="extension" title="拓展名"></vxe-column>
      <vxe-column
        field="seekname"
        title="检索名称"
        sortable
        :edit-render="{ autofocus: '.vxe-input--inner' }"
      >
        <template #edit="{ row }">
          <vxe-input v-model="row.seekname" type="text"></vxe-input>
        </template>
      </vxe-column>
    </vxe-table>
  </div>
</template>

<script lang="ts">
import {
  defineComponent,
  ref,
  onMounted,
  watch,
  computed,
  h,
  reactive,
  inject
} from "vue";
import { VXETable, VxeTableInstance, VxeTableEvents } from "vxe-table";
import { invoke } from "@tauri-apps/api/tauri";
import { ThreadPool } from '../ts/ThreadPool';
import XEUtils from 'xe-utils';
import { ElMessage } from "element-plus";

export default defineComponent({
  components: {},
  setup: function () {
    let FileData: any[] = reactive([]);
    const search = ref();

    const table = reactive({
      loading: false,
    });

    const xTable = ref({} as VxeTableInstance);


    const findList = () => {
      table.loading = true;
      return new Promise((resolve) => {
        setTimeout(() => {
          const data = FileData;
          // 阻断 vue 对大数组的监听，避免 vue 绑定大数据造成短暂的卡顿
          const $table = xTable.value;
          if ($table) {
            $table.loadData(data);
          }
          resolve(null);
          table.loading = false;
        }, 300);
      });
    };

    const searchEvent = () => {
      const $table = xTable.value;
      const searchs = XEUtils.toValueString(search.value).trim().toLowerCase();
      console.log(searchs)
      if (searchs) {
        const filterRE = new RegExp(searchs, 'gi')
        const searchProps = ['seekname', 'realname', 'extension']
        const rest = $table.getTableData().fullData.filter(item => searchProps.some(key => XEUtils.toValueString(item[key]).toLowerCase().indexOf(searchs) > -1))
        const data = rest.map(row => {
          const item = Object.assign({}, row)
          searchProps.forEach(key => {
            item[key] = XEUtils.toValueString(item[key]).replace(filterRE, match => `${match}`)
          })
          return item
        })
        $table.loadData(data);
      } else {
        $table.loadData(FileData);
      }
    }

    const getSelectEvent = () => {
      table.loading = true;
      const $table = xTable.value;
      const selectRecords = $table.getCheckboxRecords();
      let tasks: any[] = [];

      for (let select of selectRecords) {
        tasks.push(
          {
            id: select.id,
            seek_name: select.seekname,
          }
        )
      }

      invoke("change_seek_status", { smov: tasks }).then((res: any) => {
        if (res.code == 200) {
          ElMessage({
            message: '共' + tasks.length + '条加入到检索队列',
            type: 'success',
          })
        } else {
          ElMessage.error('插入到检索队列出现错误')
        }
        //table.loading = false;
      }
      ).finally(() => {
        table.loading = false;
        $table.removeCheckboxRow();
      })
    };

    function retrieveData(seekName, id, i) {
      return new ThreadPool.Task({
        params: i,
        processor: (params) => {
          // console.log("线程"+i+"正在运行");
          return new Promise(resolve => {
            console.log("正在检索", seekName);
            invoke("retrieve_data", {
              seekName: seekName,
              smovId: id,
            }).then((res) => {
              resolve(res);
            }).finally(() => {
              //resolve(params);
            });
          });
        },
        callback: (data) => {
          console.log(`线程 ${i}, rst is`, data);
        }
      });
    }

    onMounted(() => {
      initFn();
    });

    const initFn = () => {
      invoke("query_unretrieved").then((res) => {
        let data: any = res;
        if (data.code == 200) {
          FileData = data.data;
          findList();
        }
      });
    };

    //局部保存 直接修改元数据
    const editClosedEvent: VxeTableEvents.EditClosed = ({ row, column }) => {
      const $table = xTable.value;
      const field = column.property;
      const cellValue = row[field];

      //更新数据库中的数据
      invoke("update_seekname", { id: row.id, seekName: row.seekname }).then(
        (res) => {
          let data: any = res;
          if (data.code == 200) {
            if ($table.isUpdateByRow(row, field)) {
              setTimeout(() => {
                VXETable.modal.message({
                  content: `局部保存成功！ ${field}=${cellValue}`,
                  status: "success",
                });
                // 局部更新单元格为已保存状态
                $table.reloadRow(row, null, field);
              }, 300);
            }
          }
        }
      );
      // 判断单元格值是否被修改
    };

    return {
      search,
      table,
      findList,
      xTable,
      editClosedEvent,
      getSelectEvent,
      searchEvent
    };
  },
});
</script>

<style scoped lang="less">
.fileMain {
  height: 100%;
  display: flex;
  flex-flow: column nowrap;
}

.fileTable {
  flex-grow: 1;
}
</style>
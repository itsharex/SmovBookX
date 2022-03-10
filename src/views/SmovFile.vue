<template>
  <div class="fileMain">
    <div class="toolBar">
      <el-button type="primary" :icon="Files" :loading="false" @click="getSelectEvent">剑气纵横三万里</el-button>
      <el-button type="warning" :icon="Close" :loading="false" @click="changeStatusAll">一剑光寒十九洲</el-button>
      <el-button type="primary" :icon="Refresh" @click="initFn">重载</el-button>
      <el-input
        v-model="search"
        class="search"
        placeholder="全表检索"
        :suffix-icon="Search"
        @input="searchEvent"
      />
    </div>

    <vxe-table
      resizable
      show-overflow
      keep-source
      ref="xTable"
      class="fileTable"
      height="92%"
      :export-config="{}"
      :empty-render="{ name: 'NotData' }"
      :loading="table.loading"
      :checkbox-config="{ checkField: 'checked' }"
      :edit-config="{ trigger: 'dblclick', mode: 'row', showStatus: true }"
      @edit-closed="editClosedEvent"
      :scroll-y="{ gt: 100 }"
    >
      <template #empty>
        <el-empty style="line-height:50px" description="没有其他数据了哦"></el-empty>
      </template>
      <vxe-column type="seq" width="60"></vxe-column>
      <vxe-column type="checkbox" width="60"></vxe-column>
      <vxe-column field="realname" title="文件名称"></vxe-column>
      <vxe-column field="extension" title="拓展名" width="100"></vxe-column>
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
      <vxe-column field="is_active" title="可用" width="60">
        <template #default="{ row }">
          <span>{{ row.is_active == 1 ? "是" : "否" }}</span>
        </template>
      </vxe-column>
      <vxe-column title="操作" width="60">
        <template #default="{ row }">
          <el-button type="warning" :icon="Close" size="small" circle @click="changActive(row)"></el-button>
        </template>
      </vxe-column>
    </vxe-table>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref, onMounted, reactive } from "vue";
import { VXETable, VxeTableInstance, VxeTableEvents } from "vxe-table";
import { invoke } from "@tauri-apps/api/tauri";
import XEUtils from 'xe-utils';
import { ElMessage } from "element-plus";
import { Files, Refresh, Search, Close } from '@element-plus/icons-vue';

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
      const backdata = $table.getTableData().fullData;
      if (searchs) {
        const filterRE = new RegExp(searchs, 'gi')
        const searchProps = ['seekname', 'realname', 'extension']
        const rest = $table.getTableData().fullData.filter(item => searchProps.some(key => XEUtils.toValueString(item[key]).toLowerCase().indexOf(searchs) > -1))
        console.log(rest)
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
      const $table = xTable.value;
      const selectRecords = $table.getCheckboxRecords();
      let tasks: any[] = [];

      if (selectRecords.length != 0) {
        table.loading = true;
        for (let select of selectRecords) {
          tasks.push(
            {
              id: 0,
              smov_id: select.id,
              seek_name: select.seekname,
              status: 0
            }
          )
        }

        console.log("发送数据")
        console.log(Date.now())

        invoke("change_seek_status", { smov: tasks }).then((res: any) => {
          if (res.code == 200) {
            ElMessage({
              message: '共' + tasks.length + '条加入到检索队列',
              type: 'success',
            })
          } else {
            ElMessage.error('插入到检索队列出现错误,' + res.msg)
          }
        }
        ).finally(() => {
          table.loading = false;
          //使用XEUtil进行删除
          selectRecords.forEach(item => {
            XEUtils.remove(FileData, toitem => toitem === item)
          })
          $table.removeCheckboxRow();
        })
      }
    };

    //批量修改 需要优化！ 批量是否重新渲染的代价比较低
    const changeStatusAll = async () => {
      table.loading = true;
      const $table = xTable.value;
      const selectRecords = $table.getCheckboxRecords();

      const data = XEUtils.map(selectRecords, item => item.id);

      invoke("disable_smov", { id: data }).then((res: any) => {
        if (res.code == 200) {
          table.loading = false;
          $table.removeCheckboxRow();

          ElMessage({
            message: '共' + selectRecords.length + '条数据被关闭',
            type: 'success',
          })
        }
      })


      // getAllHistory(selectRecords).then((res) => {
      //   table.loading = false;
      //   $table.removeCheckboxRow();

      //   ElMessage({
      //     message: '共' + selectRecords.length + '条数据被关闭',
      //     type: 'success',
      //   })
      // })

      // initFn();
    }

    const changActive = (row: any) => {
      console.log(row.id);
      invoke("change_active_status", { id: row.id, status: 0 }).then((res: any) => {
        if (res.code == 200) {
          const $table = xTable.value;
          $table.remove(row);
          XEUtils.remove(FileData, toitem => toitem === row)
        } else {
          ElMessage.error('关闭出现了一个问题' + res.msg)
        }
      }).finally(() => {
        // resolve();
      })
    }

    const getAllHistory = async (selectRecords) => {
      let allHistory = [] as any[]
      selectRecords.forEach((item) => {
        allHistory.push((async (item) => {
          return await getChangePromise(item)
        })(item))
      })
      return await Promise.all(allHistory);
    }

    const getChangePromise = (row: any) => {
      return new Promise(function (resolve, reject) {
        invoke("change_active_status", { id: row.id, status: 0 }).then((res: any) => {
          if (res.code == 200) {
          } else {
            ElMessage.error('关闭出现了一个问题' + res.msg)
          }
        }).finally(() => {
          resolve(row);
        })
      });
    }

    onMounted(() => {
      initFn();
    });

    const initFn = () => {
      table.loading = true;
      invoke("query_unretrieved").then((res) => {
        console.log(res)
        let data: any = res;
        if (data.code == 200) {
          FileData = data.data;
          findList();
        }
      }).finally(() => {
        setTimeout(() => {
          table.loading = false;
        }, 1000);
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
      searchEvent,
      getSelectEvent,
      Files,
      Refresh,
      initFn,
      Search,
      changActive,
      Close,
      changeStatusAll
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
.toolBar {
  padding: 10px;
  border: 1px solid #e8e9eb;
  background-color: #f8f8f9;
  display: flex;
  z-index: 999;
  align-items: center;
}
</style>

<style lang="less">
.search {
  flex: 1;
  display: flex;
  justify-content: right;

  input {
    width: 250px;
  }
}
</style>



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
    <!-- 设置 overflow: overlay 滚动条浮动上层 不占用布局-->
    <vxe-table
      show-overflow
      keep-source
      ref="xTable"
      class="fileTable"
      height="92%"
      :export-config="{}"
      :loading="table.loading"
      :checkbox-config="{ checkField: 'checked' }"
      :edit-config="{ trigger: 'dblclick', mode: 'row', showStatus: true }"
      @edit-closed="editClosedEvent"
      :scroll-y="{ gt: 100 }"
    >
      <template #empty>
        <el-empty style="line-height:50px" description="没有其他数据了哦"></el-empty>
      </template>
      <vxe-column type="seq"></vxe-column>
      <vxe-column type="checkbox" width="6%"></vxe-column>
      <vxe-column field="realname" title="文件名称" width="24%"></vxe-column>
      <vxe-column field="extension" title="拓展名" width="10%"></vxe-column>
      <vxe-column
        field="seekname"
        title="检索名称(双击修改)"
        sortable
        width="33%"
        :edit-render="{ autofocus: '.vxe-input--inner' }"
      >
        <template #edit="{ row }">
          <vxe-input v-model="row.seekname" type="text"></vxe-input>
        </template>
      </vxe-column>
      <vxe-column
        field="is_active"
        title="可用"
        width="10%"
        :filter-multiple="false"
        :filters="[{ label: '可用', value: 1, checked: true }, { label: '不可用', value: 0, checked: false }]"
      >
        <template #default="{ row }">
          <span>{{ row.is_active == 1 ? "是" : "否" }}</span>
        </template>
      </vxe-column>
      <vxe-column title="操作" wwidth="5%">
        <template #default="{ row }">
          <el-button
            :type="row.is_active == 1 ? 'warning' : 'success'"
            :icon="row.is_active == 1 ? Close : Select"
            size="small"
            circle
            @click="changActive(row)"
          ></el-button>
        </template>
      </vxe-column>
    </vxe-table>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref, onMounted, reactive, inject } from "vue";
import { VXETable, VxeTableInstance, VxeTableEvents } from "vxe-table";
import { invoke } from "@tauri-apps/api/tauri";
import XEUtils from 'xe-utils';
import { ElMessage } from "element-plus";
import { Files, Refresh, Search, Close, Select } from '@element-plus/icons-vue';
import { Log } from "../type/log";
import { CurentTime } from "../util/time";

export default defineComponent({
  components: {},
  setup: function () {
    let FileData: any[] = reactive([]);
    const search = ref();

    const table = reactive({
      loading: false,
    });

    const xTable = ref({} as VxeTableInstance);

    let logs = inject('log') as any;

    const findList = () => {
      table.loading = true;
      return new Promise((resolve) => {
        setTimeout(() => {
          const data = FileData;

          console.log(FileData)
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
      const $table = xTable.value;
      const selectRecords = $table.getCheckboxRecords();

      if (selectRecords.length > 0) {
        table.loading = true;
        const data = XEUtils.map(selectRecords, item => {
          const a = {
            id: item.id,
            is_active: item.is_active === 0 ? 1 : 0
          };
          return a
        });

        invoke("disable_smov", { id: data }).then((res: any) => {
          if (res.code == 200) {
            let enable = 0;
            let isActive = 0;
            XEUtils.arrayEach(selectRecords, (item, key) => {
              if (item.is_active === 0) {
                enable = enable + 1;
                isActive = 1;
              } else {
                isActive = 0;
              }
              item.is_active = isActive;
              $table.reloadRow(item, null, "is_active");
            })
            table.loading = false;
            let msg = "共" + (selectRecords.length - enable) + "条数据被关闭," + enable + "条数据开启";
            const log: Log = {
              time: CurentTime(),
              thread: 'SmovFile.vue',
              msg: msg,
              level: 'INFO'
            }
            logs.value = log;
            ElMessage({
              message: msg,
              type: 'success',
            })
          }
        })

      }

    }

    const changActive = (row: any) => {

      const isActive = row.is_active == 1 ? 0 : 1;

      invoke("change_active_status", { id: row.id, status: isActive }).then((res: any) => {
        if (res.code == 200) {
          const $table = xTable.value;
          // $table.remove(row);
          row.is_active = isActive;
          $table.reloadRow(row, null, undefined)
          // XEUtils.remove(FileData, toitem => toitem === row)
        } else {
          ElMessage.error('关闭出现了一个问题' + res.msg)
        }
      }).finally(() => {

      })
    }

    onMounted(() => {
      initFn();
    });

    const initFn = () => {
      table.loading = true;
      invoke("query_unretrieved").then((res) => {
        // console.log(res)
        let data: any = res;
        if (data.code == 200) {
          FileData = data.data;
          findList();
        }
      }).finally(() => {
        setTimeout(() => {
          table.loading = false;
        }, 500);

      });
    };

    //局部保存 直接修改元数据  经过测试 除了删除其他操作数据都会同步到元数据
    const editClosedEvent: VxeTableEvents.EditClosed = ({ row, column }) => {
      const $table = xTable.value;
      const field = column.field;  //旧值 
      console.log(field)
      const cellValue = row[field]; //新值

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
      changeStatusAll,
      Select
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

<style lang="less">
//处理滚动条溢出
.vxe-table--render-default .vxe-table--body-wrapper,
.vxe-table--render-default .vxe-table--footer-wrapper,
.vxe-table--render-default .vxe-table--header-wrapper {
  overflow: overlay;
}
</style>


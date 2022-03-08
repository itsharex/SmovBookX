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

    <!-- <vxe-toolbar perfect>
   
    <template #buttons>-->
    <!-- <vxe-button type="text" icon="fa fa-plus" content="加入检索列表" @click="getSelectEvent"></vxe-button> -->
    <!-- <el-button type="primary" :icon="Files" :loading="false">加入检索列表</el-button> -->
    <!-- <vxe-input v-model="search" type="search" placeholder="全表搜索" @keyup="searchEvent"></vxe-input> -->
    <!-- <vxe-button type="text" icon="fa fa-trash-o" content="删除"></vxe-button>
    <vxe-button type="text" icon="fa fa-save" content="保存"></vxe-button>-->
    <!-- </template>
    </vxe-toolbar>-->

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
        <!-- <div class="noData">
          <img src="https://pic2.zhimg.com/50/v2-f7031359103859e1ed38559715ef5f3f_hd.gif" />
          <p>亲，没有更多数据了！</p>
        </div>-->
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
          <!-- <el-switch
              v-model="row.is_active"

              :active-value="1"
              :inactive-value="0"
              @change="changActive(row)"
          />-->
          <!-- <vxe-switch v-model="row.is_active" :open-value="1"  :close-value="2" @change="changActive(row)" ></vxe-switch> -->
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
import { ThreadPool } from '../ts/ThreadPool';
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

    // VXETable.renderer.add('NotData', {
    //   // 空内容模板
    //   renderEmpty() {
    //     return [<span><img src='https://pic2.zhimg.com/50/v2-f7031359103859e1ed38559715ef5f3f_hd.gif'/> <p>亲，没有更多数据了！</p> </span>]
    //   }
    // })

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

        invoke("change_seek_status", { smov: tasks }).then((res: any) => {
          if (res.code == 200) {
            ElMessage({
              message: '共' + tasks.length + '条加入到检索队列',
              type: 'success',
            })
          } else {
            ElMessage.error('插入到检索队列出现错误,' + res.msg)
          }
          //是否有删除原始数据的更优解法
          //initFn();
          //table.loading = false;
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
    const changeStatusAll = () => {
      table.loading = true;
      const $table = xTable.value;
      const selectRecords = $table.getCheckboxRecords();
      selectRecords.forEach(item => {
        // invoke("change_active_status", { id: item.id, status: 0 }).then((res: any) => {
        //   if (res.code == 200) {
        //     const $table = xTable.value;
        //   } else {
        //     ElMessage.error('关闭出现了一个问题' + res.msg)
        //   }
        // })
        changActive(item);
      });

      table.loading = false;

      ElMessage({
        message: '共' + selectRecords.length + '条数据被关闭',
        type: 'success',
      })
      // initFn();
    }

    const changActive = (row) => {
      invoke("change_active_status", { id: row.id, status: 0 }).then((res: any) => {
        if (res.code == 200) {
          const $table = xTable.value;
          $table.remove(row);
          XEUtils.remove(FileData, toitem => toitem === row)
        } else {
          ElMessage.error('关闭出现了一个问题' + res.msg)
        }
      })

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

<template>
  <div>
    <!-- <el-empty description="未检测到文件" v-if="FileData === undefined || FileData.length === 0"></el-empty>
    <el-skeleton v-if="skeleton" />-->
    <!-- <smov-list v-for="(data,index) in FileData "
               :data="data"
               :index="index"
               :key="index"
               @ChangeStatus="ParenStatus"
               @ChangeName="changeName">
    </smov-list>-->

    <!-- <el-table :data="FileData" style="width: 100%" :row-key="id">
      <el-table-column type="selection" width="55" />
      <el-table-column type="index" fixed prop="realname" label="检索名" width="300" >
         <template #default="scope">
          <span>{{scope.row.realname}}</span>
          <el-input v-model="FileData[scope.$index].realname" placeholder="Please input" />
        </template>
        </el-table-column>
      <el-table-column prop="path" label="路径" />
      <el-table-column prop="len" label="文件大小" />
      <el-table-column fixed="right" label="操作">
        <template #default="scope">
          <el-button type="text" size="small" @click="handleClick">详情</el-button>
          <el-button type="text" size="small" @click="handleEdit(scope.$index, scope.row)">修改</el-button>
        </template>
      </el-table-column>
    </el-table>-->

    <vxe-toolbar export :refresh="{ query: findList }">
      <template #buttons>
        <vxe-button @click="getSelectEvent">获取选中</vxe-button>
      </template>
    </vxe-toolbar>

    <vxe-table
      resizable
      show-overflow
      keep-source
      ref="xTable"
      height="500"
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

    <!-- <el-dialog v-model="centerDialogVisible" title="Tips" width="30%" :before-close="handleClose">
      <template #footer>  
        <span class="dialog-footer">
          <el-button @click="centerDialogVisible = false">Cancel</el-button>
          <el-button type="primary" @click="dialogFinish">Confirm</el-button>
        </span>
      </template>
    </el-dialog>-->

    <div class="SmovFileNav"></div>

    <div class="navBottom">
      <el-input
        size="small"
        style="width: 200px"
        v-model="search"
        v-if="false"
      />
      <!-- <el-button size="small" @click="initFn">检索全部</el-button>

      <el-button size="small" @click="SearchFile">db数据检索</el-button>-->
    </div>
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
} from "vue";
import { VXETable, VxeTableInstance, VxeTableEvents } from "vxe-table";
import SmovList from "../components/SmovList.vue";
import { invoke } from "@tauri-apps/api/tauri";
import { ElNotification } from "element-plus";

export default defineComponent({
  components: { SmovList },
  setup: function () {
    const skeleton = ref(false);
    const empty = ref(false);
    let FileData: any[] = reactive([]);
    const search = ref();
    const singleData = ref({});
    const centerDialogVisible = ref(false);

    const table = reactive({
      loading: false,
    });

    const xTable = ref({} as VxeTableInstance);

    const mockList = (size: number) => {
      const list: any[] = [];
      for (let index = 0; index < size; index++) {
        list.push({
          checked: false,
          realname: `名称${index}`,
          sex: "0",
          num: 123,
          age: 18,
          num2: 234,
          rate: 3,
          address: "shenzhen",
        });
      }
      return list;
    };

    const findList = () => {
      table.loading = true;
      console.log(FileData);
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

    const getSelectEvent = () => {
      const $table = xTable.value;
      const selectRecords = $table.getCheckboxRecords();
      // const allRecords = $table.getTableData().fullData;//获取全部数据

      // VXETable.modal.alert(`${selectRecords.length}条数据`)

      for (let select of selectRecords) {
        retrieveData(select.seekname, select.id);  //await 关键词 等待完成
      }
    };

    async function retrieveData(seekName, id) {
      invoke("retrieve_data", {
        seekName: seekName,
        smovId: id,
      }).then((res) => {
        console.log(res);
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

    const ParenStatus = (index, status) => {
      FileData[index].status = status;
    };

    const changeName = (index, name) => {
      console.log(name);
      FileData[index].formatCode = name;
    };
    const filter = () => {
      return FileData.filter((item) => item.name.indexOf(search) < 0);
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

    const handleEdit = (index, row) => {
      const data = {
        index: index,
        data: row,
      };

      singleData.value = data;

      centerDialogVisible.value = true;

      ElNotification({
        title: "Title",
        message: h("i", { style: "color: teal" }, "This is a reminder"),
      });
    };

    const dialogFinish = () => {
      centerDialogVisible.value = false;
    };

    return {
      skeleton,
      empty,
      FileData,
      ParenStatus,
      changeName,
      filter,
      search,
      initFn,
      handleEdit,
      centerDialogVisible,
      dialogFinish,
      table,
      findList,
      xTable,
      editClosedEvent,
      getSelectEvent,
    };
  },
});
</script>

<style>
/* .vxe-table--empty-placeholder /deep/ .vxe-loading {
  background-color: none;
} */
</style>
<template>

  <el-card class="SmovList "
           :class="[data.status === 1 ? 'el-card_normal' : data.status === 2 ? 'el-card_success' : data.status === 3 ? 'el-card_nodata' : 'el-card_normal']"
           v-loading="data.loading">
    <p class="name slChild" v-if="!NameEdit">{{ data.formatCode }}</p>
    <el-input v-model="Name" v-if="NameEdit" class="name slChild" @blur="blur" size="small"/>
    <p class="path slChild"><span>&nbsp;&nbsp;&nbsp;&nbsp;</span>路径: {{ data.path }}</p>
    <div class="editButton">
      <el-button type="primary" v-if="((data.status === 3 ||  data.status === 1)&& afterChange === false) "
                 @click="click" :icon="Edit" circle></el-button>
      <el-button type="success" :icon="Check" v-if="afterChange === true" @click="pushSingeData" circle></el-button>
      <el-button type="danger" :icon="Close" v-if="afterChange === true" @click="back" circle></el-button>
    </div>
  </el-card>
</template>

<script>
import {defineComponent, ref} from 'vue'
import {Edit, Check, Close} from '@element-plus/icons-vue'

export default defineComponent({
  name: "SmovList",
  props: ['data', 'index'],
  setup(props, {emit}) {

    const loading = ref(false);
    const NameEdit = ref(false);
    const Name = ref("");
    const afterChange = ref(false);

    const blur = () => {
      afterChange.value = true
    };

    const pushSingeData = () => {
      afterChange.value = false;
      NameEdit.value = false;
      emit('ChangeName', props.index, Name);
      if (props.data.status === 1) {
        return;
      } else {
        emit('ChangeStatus', props.index, 2);
      }
    };
    const back = () => {
      afterChange.value = false
      NameEdit.value = false;
    };
    const click = () => {
      afterChange.value = true;
      NameEdit.value = true;
      Name.value = props.data.formatCode
    };
    return {
      loading,
      click,
      Edit,
      NameEdit,
      Name,
      blur,
      afterChange,
      Check,
      pushSingeData,
      back,
      Close
    };
  },
})
</script>

<style lang="less" scoped>
.SmovList {
  margin-top: 10px;
}


.el-card {
  align-content: center;
}

//未查询到状态
.el-card_nodata {
  background-color: var(--el-color-danger-lighter);
  border-left: 5px solid var(--el-color-danger);
}

//正常状态
.el-card_normal {
  background-color: var(--el-color-info-lighter);
  border-left: 5px solid var(--el-color-info);
}

//完成查询状态
.el-card_success {
  background-color: #f0f9eb;
  border-left: 5px solid var(--el-color-success);
}

.el-card__body {
  padding: 12px;
}

.name {
  font-size: 16px;
  font-weight: bold;
  width: 30%;
}

.path {
  font-size: 14px;
  font-weight: normal;
  color: #8c939d;
  margin: 0 auto;
}

.slChild {
  text-align: left;
  margin: 7px;
  vertical-align: middle;
}

.editButton {
  flex: 1;
  align-self: flex-end;
  display: flex;
  flex-direction: row-reverse;

  > * {
    margin-left: 10px;
  }
}

</style>

<style lang="less">
.el-card__body {
  display: flex;
  flex-direction: row;
}
</style>

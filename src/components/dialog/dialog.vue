<template>
  <el-dialog
    :title="title"
    v-model="dialogVisible"
    width="30%"
    @close="close">
    <span>{{content}}</span>
    <template #footer>
      <span class="dialog-footer">
        <el-button @click="close">取 消</el-button>
        <el-button type="primary" @click="close">确 定</el-button>
      </span>
    </template>
  </el-dialog>
</template>

<script lang="ts">
import { defineComponent } from 'vue'
export default defineComponent({
  props: {
    close: {
      type: Function as any
    },
    content: {
      type: String,
      default: '暂无权限'
    },
    title: {
      type: String,
      default: '提示'
    }
  },
  setup () {
    return {
      dialogVisible: true
    }
  }
})
</script>
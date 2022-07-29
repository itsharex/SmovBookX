<template>
  <div>
    <el-dialog
      v-model="dialogVisible"
      width="40%"
      @close="close"
      :show-close="false"
    >
      <template #title="{ close, titleId, titleClass }">
        <!-- 因为怎么加字怎么别扭 就不加了 -->
        <el-button type="danger" @click="close" v-show="false">
          Close
        </el-button>
        <div class="my-header" v-show="false">
          <p :id="titleId" :class="titleClass">{{ title }}</p>
        </div>
      </template>
      <div class="qr" id="qrcode" ref="canvas"></div>
      <template #footer>
        <span class="dialog-footer"> </span>
      </template>
    </el-dialog>
  </div>
</template>

<script lang="ts">
import { defineComponent, nextTick } from "vue";
import { qrcanvas, QRCanvasEffect } from "qrcanvas";
import { CircleCloseFilled } from "@element-plus/icons-vue";
export default defineComponent({
  props: {
    close: {
      type: Function as any,
    },
    qr: {
      type: String,
      default: "",
    },
    title: {
      type: String,
      default: "提示",
    },
  },
  setup(props) {
    nextTick(() => {
      const colorFore = "#e6c039";
      const colorOut = "#c7415b";
      const colorIn = "#c7415b";
      const canvas = qrcanvas({
        data: props.qr,
        correctLevel: "Q",
        cellSize: 30,
        resize: true,
        foreground: [
          // foreground color
          { style: colorFore },
          // outer squares of the positioner
          { row: 0, rows: 7, col: 0, cols: 7, style: colorOut },
          { row: -7, rows: 7, col: 0, cols: 7, style: colorOut },
          { row: 0, rows: 7, col: -7, cols: 7, style: colorOut },
          // inner squares of the positioner
          { row: 2, rows: 3, col: 2, cols: 3, style: colorIn },
          { row: -5, rows: 3, col: 2, cols: 3, style: colorIn },
          { row: 2, rows: 3, col: -5, cols: 3, style: colorIn },
        ],
      });

      document.getElementById("qrcode")!.innerHTML = "";
      document.getElementById("qrcode")!.appendChild(canvas);
    });

    const dialogVisible = ref(true);

    return {
      dialogVisible,
      CircleCloseFilled,
    };
  },
});
</script>

<style lang="less" scoped>
.qr {
  width: 100%;
  height: 300px;
  display: flex;
  justify-content: center;
  align-content: center;

  canvas {
    border: black solid 10px;
  }
}

.QrMain {
  border-radius: 1rem;
}

.my-header {
  height: 1.5rem;

  p {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    font-size: 1.1rem;
    font-weight: 500;
  }
}
</style>
<template>
  <el-dialog :title="title" v-model="dialogVisible" width="40%" @close="close">
    <div class="qr" id="qrcode"></div>
    <template #footer>
      <span class="dialog-footer">
      </span>
    </template>
  </el-dialog>
</template>

<script lang="ts">
import { defineComponent, nextTick } from 'vue'
import { qrcanvas, QRCanvasEffect } from "qrcanvas";
export default defineComponent({
  props: {
    close: {
      type: Function as any
    },
    qr: {
      type: String,
      default: ''
    },
    title: {
      type: String,
      default: '提示'
    }
  },
  setup(props) {
    nextTick(() => {
      const colorFore = '#55a';
      const colorOut = '#c33';
      const colorIn = '#621';
      const canvas = qrcanvas({
        data: props.qr,
        correctLevel: 'Q',
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
    return {
      dialogVisible: true
    }
  }
})
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
</style>
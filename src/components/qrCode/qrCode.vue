<template>
  <div>
    <el-dialog
      v-model="dialogVisible"
      width="30%"
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
        <div class="qr-footer">
          <el-tooltip placement="top" content="点按复制" effect="customized">
            <span class="dialog-footer" @click="copyUrl">{{ qr }}</span>
          </el-tooltip>
        </div>
      </template>
    </el-dialog>
  </div>
</template>

<script lang = "ts" setup>
import { defineComponent, nextTick } from "vue";
import { qrcanvas, QRCanvasEffect } from "qrcanvas";
import { CircleCloseFilled } from "@element-plus/icons-vue";
import { CopyLink } from "@icon-park/vue-next";
import { ElMessage } from "element-plus";
import useClipboard from "vue-clipboard3";
import {alert} from "@/util/alert";


const props = defineProps({
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
});

nextTick(() => {
  const colorFore = "#000066";
  const colorOut = "#000000";
  const colorIn = "#000000";
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

const { toClipboard } = useClipboard();

const copyUrl = async () => {
  try {
    await toClipboard(props.qr);
    alert.success("复制成功");
    
  } catch (e) {
    console.error(e);
    alert.error("复制失败");
  }
};

const dialogVisible = ref(true);
</script>

<style lang="less" scoped>
.qr {
  width: 100%;
  height: 200px;
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

.qr-footer {
  width: 100%;
  display: flex;
  justify-content: center;
  align-content: center;
  flex-direction: column;
  cursor: pointer;
  .dialog-footer {
    font-weight: 600;
    text-align: center;
  }
  .dialog-footer:hover {
    color: #4a4a4a;
  }
  .url-copy {
    margin-left: 12px;
    text-align: center;
    display: block;
  }
  .url-copy:hover {
    color: rgb(0, 0, 0);
  }
}
</style>

<style >
.el-popper.is-customized {
  /* Set padding to ensure the height is 32px */
  padding: 6px 12px;
  background: linear-gradient(90deg, rgb(159, 229, 151), rgb(204, 229, 129));
}

.el-popper.is-customized .el-popper__arrow::before {
  background: linear-gradient(45deg, #b2e68d, #bce689);
  right: 0;
}
</style>
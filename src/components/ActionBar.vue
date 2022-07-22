<template>
  <div class="ActionBar" data-tauri-drag-region>
    <slot></slot>
    <div class="WindowAction">
      <action-bar-button
        v-show="top && !AlwaysOnTop"
        @click="ChangeAlwaysOnTop()"
      >
        <pin theme="outline" size="16" fill="#333" />
      </action-bar-button>

      <action-bar-button
        v-show="top && AlwaysOnTop"
        @click="ChangeAlwaysOnTop()"
      >
        <pin theme="filled" size="16" fill="#333" />
      </action-bar-button>

      <action-bar-button v-if="minImize" @click="window.minimize()">
        <minus theme="outline" size="16" fill="#333" />
      </action-bar-button>

      <action-bar-button v-if="imize" @click="window.toggleMaximize()">
        <copy theme="outline" size="16" fill="#333" />
      </action-bar-button>

      <action-bar-button @click="once ? window.close() : window.hide()">
        <close theme="filled" size="16" fill="#333" strokeLinejoin="bevel" />
      </action-bar-button>
    </div>
  </div>
</template>

<script lang='ts' setup>
import {
  CloseBold,
  SemiSelect,
  Promotion,
  Place,
  CircleCheck,
  CircleCheckFilled,
} from "@element-plus/icons-vue";
import { getCurrent } from "@tauri-apps/api/window";
import { Copy, Close, Minus, Lock, Pin } from "@icon-park/vue-next";

const window = getCurrent();

//这个值需要保存到顶部 让所有组件都能访问到
const AlwaysOnTop:any = inject("AlwaysOnTop");

const ChangeAlwaysOnTop = () => {
  AlwaysOnTop.value = !AlwaysOnTop.value;
  window.setAlwaysOnTop(AlwaysOnTop.value);
};

const props = defineProps({
  imize: {
    type: Boolean,
    default: true,
  },
  close: {
    type: Boolean,
    default: true,
  },
  minImize: {
    type: Boolean,
    default: true,
  },
  top: {
    type: Boolean,
    default: false,
  },
  once: {
    type: Boolean,
    default: false,
  },
});
</script>
<style lang='less' scoped>
.ActionBar {
  width: 100%;
  height: 100%;
  display: flex;
  justify-content: right;
  align-items: center;
  z-index: 9999;
  & > * {
    cursor: pointer;
  }
}

.WindowButton {
  width: 2.8rem;
  height: 100%;
}

.WindowButton:hover {
  background-color: rgba(0, 0, 0, 0.144);
}

.WindowAction {
  height: 100%;
  display: flex;
  justify-content: center;
  align-items: center;
}
</style>
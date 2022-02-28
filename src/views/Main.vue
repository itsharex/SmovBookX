<template>
  <div class="app">
    <el-container>
      <el-header>
        <Navigation />
      </el-header>
      <el-main>
        <router-view />
      </el-main>
      <el-footer></el-footer>
    </el-container>
    <Log />
  </div>
</template>

<script lang="ts">
import { defineComponent, ref, onMounted, inject, watch, computed } from "vue";
import Log from '../components/Log.vue';
import Navigation from "../components/Navigation.vue";

export default defineComponent({
  components: { Log, Navigation },
  setup() {

    return {

    };
  },
});

</script>

<style lang="less">
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  padding: 0%;
}

body {
  margin: 0;
}

.app {
  display: flex;
  height: 100vh;
  flex-flow: row nowrap;
}

::-webkit-scrollbar {
  height: 9px;
  width: 8px;
}

::-webkit-scrollbar-thumb {
  border-radius: 10px;
  border-style: dashed;
  border-color: transparent;
  border-width: 2px;
  background-color: rgba(157, 165, 183, 0.4);
  background-clip: padding-box;
}

::-webkit-scrollbar-thumb:hover {
  border-width: 1px;
  background: rgba(157, 165, 183, 0.4);
}

.app .el-header,
.common-layout .el-footer {
  background-color: #cfdbeb;
  color: var(--el-text-color-primary);
  line-height: 60px;
  padding: 0;
}
.app .el-footer {
  line-height: 60px;
  height: 3em;
}

.app .el-aside {
  background-color: #d3dce6;
  color: var(--el-text-color-primary);
  text-align: center;
  line-height: 200px;
}

.app .el-main {
  color: var(--el-text-color-primary);
  text-align: center;
  line-height: 160px;
  flex-grow: 1;
}

.app > .el-container {
  height: 100%;
}
</style>




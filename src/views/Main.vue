<template>
  <div class="app">
    <el-container>
      <el-header height="6vh">假装有个系统栏</el-header>
      <el-container>
        <el-aside class="NavAside" width="170px">
          <Navigation />
        </el-aside>
        <el-container>
          <el-main class="smovMain" id="smovMain">
            <router-view />
          </el-main>

          <el-footer height="3em"></el-footer>
        </el-container>
      </el-container>
    </el-container>
    <Log />
  </div>
</template>

<script lang="ts">
import { defineComponent, ref, onMounted, provide, inject, watch, computed, reactive } from "vue";
import Navigation from "../components/Navigation.vue";
import Log from "../components/Log.vue";

export default defineComponent({
  components: { Navigation, Log },
  setup() {

    return {

    };
  },
});


function unknow(unknow: any) {
  throw new Error("Function not implemented.");
}
</script>

<style lang="less">
#app {
  // font-family: Avenir, Helvetica, Arial, sans-serif;
  font-family: "Microsoft Yahei", "PingFang SC", "system-ui";
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  padding: 0%;
  background-color: #00000000;
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
  margin-right: 1px;
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

.smovMain {
   height: 82vh;
}
</style>




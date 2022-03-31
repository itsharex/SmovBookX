<template>
  <div class="app">
    <el-container>
      <el-header class="WindowHeader" height="40px" data-tauri-drag-region >
        <action-bar data-tauri-drag-region >
          cesh
          </action-bar>
      </el-header>
      <el-container>
        <el-aside class="NavAside" width="180px">
          <Navigation />
        </el-aside>
        <el-container>
          <el-main class="SmovMain" id="SmovMain">
            <router-view v-slot="{ Component }">
              <keep-alive :max="2">
                <component :is="Component" v-if="$route.meta.keepAlive" :key="$route.name" />
              </keep-alive>
              <component :is="Component" v-if="!$route.meta.keepAlive" :key="$route.name" />
            </router-view>
          </el-main>
          <el-footer v-if="log" height="3em"></el-footer>
        </el-container>
      </el-container>
    </el-container>
    <Log v-if="log" />
  </div>
</template>

<script lang="ts" setup>

const log = false;

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

.SmovMain {
  max-height: calc(~"100vh - 50px") ;
}

.SmovMainHeightHaslog {
  max-height: calc(~"100vh - 50px") ;
}

.SmovMainHeightUnlog {
  max-height: calc(~"100vh - 50px - 3em ") ;
}

.NavAside {
  background-color: rgba(240, 240, 240, 0.459);
  border-radius: 0 7px 0 0;
  // padding: 2px;
  // box-shadow: var(--el-box-shadow-light) inset;
  // border: #2c3e502d solid 2px;
}
.WindowHeader {
  background: #6a626251;
}
</style>




import { ElLoading } from "element-plus";

<template>
  <div class="app">
    <el-container>
      <el-header class="WindowHeader" height="40px">
        <action-bar data-tauri-drag-region>
          <div class="quickButton" v-if="false">
          </div>
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
import { ElLoading } from 'element-plus';
import 'element-plus/es/components/loading/style/css'
import { checkUpdate, installUpdate } from '@tauri-apps/api/updater';
import { relaunch } from '@tauri-apps/api/process';
import { StarFilled } from '@element-plus/icons-vue';
const log = false;

const Updater: any = ref({})

const UpdatePopover = ref({
  Loading: false,
  show: false
});

//增加控制 是否自动检测版本更新
const linstenUpdate = async () => {
  Updater.value = await checkUpdate();

  console.log(Updater.value)
}

const install = async () => {
  const loading = ElLoading.service({
    lock: true,
    text: '正在下载更新，下载完成后会自动更新',
  })
  await installUpdate();
  await relaunch();
}

linstenUpdate();

</script>

<style lang="less">
#app {
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
  max-height: calc(~"100vh - 50px");
}

.SmovMainHeightHaslog {
  max-height: calc(~"100vh - 50px");
}

.SmovMainHeightUnlog {
  max-height: calc(~"100vh - 50px - 3em ");
}

.NavAside {
  background-color: rgba(240, 240, 240, 0.459);
}
.WindowHeader {
  background-color: rgba(240, 240, 240, 0.459);
  padding: 0;
}

.quickButton {
  height: 100%;
  width: 60px;
  display: flex;
  justify-content: center;
  align-items: center;
  margin-left: 1rem;
}

.quickButton:hover {
  background: #9f9f9fba;
}

.updateIco {
  height: 100%;
  width: 20px;
}
</style>




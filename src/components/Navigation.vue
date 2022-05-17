<template>
  <div class="Navigation">
    <div class="title">
      <p>Smov Book</p>
    </div>

    <div class="NavItemDiv">
      <navigation-item
        v-for="(item, index) in nav.list"
        :key="index"
        :choose="nav.choose"
        :name="item.name"
        :path="item.path"
        :index="index"
        @change-choose="changeChoose"
        :show="item.show"
        :ico="item.ico"
      >
      </navigation-item>
    </div>

    <div class="QuickButton">
      <div>
        <navigation-item
          :choose="nav.choose"
          :name="'列表'"
          :index="-2"
          :show="true"
          @click="goSeek"
          :class="onLoad ? 'bg-liuguang' : ''"
          :ico="HelpFilled"
        >
          <el-icon
            :size="17"
            class="loadIco"
            :class="onLoad ? 'onLoad' : ''"
            v-if="false"
          >
            <component :is="Loading"></component>
          </el-icon>
        </navigation-item>
      </div>

      <div>
        <navigation-item
          :choose="nav.choose"
          :name="'HFS'"
          :index="-3"
          :show="true"
          @click.stop="hfs"
          :class="HfsStatus == 2 ? 'bg-liuguang' : ''"
          :ico="Platform"
        >
          <div class="qrScann">
            <el-icon :size="17" class="loadIco onLoad" v-if="HfsStatus == 1">
              <component :is="Loading"></component>
            </el-icon>
            <el-icon :size="17" v-if="HfsStatus == 2" @click.stop="qrOpen">
              <component :is="Iphone"></component>
            </el-icon>
          </div>
        </navigation-item>
      </div>

      <div class="UploadFilled">
        <el-popover
          placement="right"
          title="更新"
          :width="200"
          trigger="hover"
          v-model:visible="UpdatePopover.show"
          v-if="Updater.shouldUpdate"
        >
          <p>检测到新的更新，点击当前按钮进行更新</p>
          <p class="Version">版本号:{{ Updater.manifest.version }}</p>
          <div style="text-align: right; margin: 0"></div>
          <template #reference>
            <navigation-item
              :choose="nav.choose"
              :name="'更新'"
              :index="-2"
              :show="true"
              @mouseover="UpdatePopover.show = true"
              @mouseleave="UpdatePopover.show = false"
              @click="install"
              :ico="UploadFilled"
            ></navigation-item>
          </template>
        </el-popover>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref, onMounted, inject, markRaw } from "vue";
import { useRouter } from "vue-router";
import { getCurrent, WebviewWindow } from "@tauri-apps/api/window";
import { listen, emit } from "@tauri-apps/api/event";
import {
  ArrowLeftBold,
  Loading,
  Download,
  Cloudy,
  HomeFilled,
  Menu,
  Tools,
  UploadFilled,
  HelpFilled,
  Platform,
  Iphone,
} from "@element-plus/icons-vue";
import { checkUpdate, installUpdate } from "@tauri-apps/api/updater";
import { relaunch } from "@tauri-apps/api/process";
import { ElLoading } from "element-plus";
import "element-plus/es/components/loading/style/css";
import { request } from "../util/invoke";
import mountContent from "./qrCode/qrCode";
import { alert } from "../util/alert";
export default defineComponent({
  name: "Navigation",
  props: [],
  setup(props, { emit }) {
    const router = useRouter();
    const back = () => {
      router.back();
    };

    let webview: any = null;

    const Updater: any = ref({});

    const HfsStatus: any = ref(0);

    const UpdatePopover = ref({
      Loading: false,
      show: false,
    });

    const nav = ref({
      choose: 0,
      list: [
        { name: "首页", path: "/SomvView", ico: markRaw(HomeFilled) },
        { name: "检索", path: "/SomvFile", ico: markRaw(Menu) },
        { name: "设置", path: "/setting", ico: markRaw(Tools) },
        {
          name: "首页",
          path: "/index",
          show: process.env.NODE_ENV === "development",
        },
        {
          name: "测试",
          path: "/test",
          show: process.env.NODE_ENV === "development",
        },
      ] as any[],
    } as any);

    onMounted(() => {
      webview = new WebviewWindow("seek", {
        url: "/seek",
        title: "检索列表",
        width: 500,
        height: 700,
        center: true,
        visible: false,
        alwaysOnTop: false,
        skipTaskbar: true,
        resizable: false,
        decorations: false,
      });

      webview.once("tauri://created", function () {
        request("set_style", { effect: "", label: "seek" });
      });

      eventSeekStatus();
      linstenUpdate();
      eventHfsStatus();
    });

    //增加控制 是否自动检测版本更新
    const linstenUpdate = async () => {
      Updater.value = await checkUpdate();

      console.log(Updater.value);
    };

    const eventHfsStatus = async () => {
      !(async () =>
        await listen("HFS://OperatingStatus", (event: any) => {
          console.log(event);
          if (event.payload.data) {
            HfsStatus.value = 2;
          } else {
            HfsStatus.value = 0;
          }

          if (event.payload.code != 200) {
            alert.error(event.payload.msg);
          }
        }))();
    };

    const eventSeekStatus = async () => {
      !(async () =>
        await listen("seek_status", (event: any) => {
          console.log(event.payload);
          onLoad.value = eval(event.payload.toLowerCase());
          if (event.payload.code != 200) {
            alert.error(event.payload.msg);
          }
        }))();
    };

    const install = async () => {
      const loading = ElLoading.service({
        lock: true,
        text: "正在下载更新，下载完成后会自动更新",
      });
      await installUpdate();
      await relaunch();
    };

    const changeChoose = (index: any) => {
      nav.value.choose = index;
    };

    const onLoad = ref(false);

    const goSeek = () => {
      webview.show();
      webview.unminimize();

      webview.setAlwaysOnTop(true);
      setTimeout(() => {
        webview.setAlwaysOnTop(false);
      }, 50);
    };

    const hfs = () => {
      if (HfsStatus.value != 1) {
        const commond =
          HfsStatus.value == 0 ? "rocket_main" : "request_shutdown";
        if (HfsStatus.value == 0) {
          request("run_hfs");
        } else {
          emit("HFS://ShutDown");
        }

        HfsStatus.value = 1;
      }
    };

    const qrOpen = () => {
      request("get_local_ip").then((res: any) => {
        mountContent({ title: "手机端扫描", qr: res.data + ":8000" });
      });
    };

    return {
      ArrowLeftBold,
      back,
      goSeek,
      Download,
      onLoad,
      Loading,
      Cloudy,
      Updater,
      UpdatePopover,
      install,
      router,
      nav,
      changeChoose,
      Tools,
      UploadFilled,
      HelpFilled,
      Platform,
      HfsStatus,
      hfs,
      Iphone,
      qrOpen,
    };
  },
});
</script>

<style scoped lang="less">
.Navigation {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  z-index: 99999;
  align-items: center;
  padding: 5px;
  box-sizing: border-box;
  height: 100%;
}

@keyframes rotating {
  from {
    transform: rotate(0);
  }

  to {
    transform: rotate(360deg);
  }
}

.onLoad {
  animation: rotating 3s linear infinite;
}

.Version {
  font-weight: 600;
}

.NavItemDiv {
  width: 100%;
}

.title {
  p {
    font-size: 1.5rem;
    font-weight: 600;
    width: 100%;
    text-align: center;
  }
}

.ChooseItem {
  position: absolute;
}

.QuickButton {
  flex-grow: 1;
  display: flex;
  flex-direction: column-reverse;
  margin-bottom: 12px;
  width: 100%;

  .setting {
    width: 100%;
  }
}

.loadIco {
  margin-left: 1rem;
}
</style>

<style>
.bg-liuguang {
  animation: liuguang 2s infinite linear;
}

@keyframes liuguang {
  0% {
    background: linear-gradient(
      to bottom right,
      #8a8af4 0%,
      #ffa0ec 100%,
      #8a8af4 200%,
      #ffa0ec 300%
    );
  }

  5% {
    background: linear-gradient(
      to bottom right,
      #8a8af4 -10%,
      #ffa0ec 90%,
      #8a8af4 190%,
      #ffa0ec 290%
    );
  }

  10% {
    background: linear-gradient(
      to bottom right,
      #8a8af4 -20%,
      #ffa0ec 80%,
      #8a8af4 180%,
      #ffa0ec 280%
    );
  }

  15% {
    background: linear-gradient(
      to bottom right,
      #8a8af4 -30%,
      #ffa0ec 70%,
      #8a8af4 170%,
      #ffa0ec 270%
    );
  }

  20% {
    background: linear-gradient(
      to bottom right,
      #8a8af4 -40%,
      #ffa0ec 60%,
      #8a8af4 160%,
      #ffa0ec 260%
    );
  }

  25% {
    background: linear-gradient(
      to bottom right,
      #8a8af4 -50%,
      #ffa0ec 50%,
      #8a8af4 150%,
      #ffa0ec 250%
    );
  }

  30% {
    background: linear-gradient(
      to bottom right,
      #8a8af4 -60%,
      #ffa0ec 40%,
      #8a8af4 140%,
      #ffa0ec 240%
    );
  }

  35% {
    background: linear-gradient(
      to bottom right,
      #8a8af4 -70%,
      #ffa0ec 30%,
      #8a8af4 130%,
      #ffa0ec 230%
    );
  }

  40% {
    background: linear-gradient(
      to bottom right,
      #8a8af4 -80%,
      #ffa0ec 20%,
      #8a8af4 120%,
      #ffa0ec 220%
    );
  }

  45% {
    background: linear-gradient(
      to bottom right,
      #8a8af4 -90%,
      #ffa0ec 10%,
      #8a8af4 110%,
      #ffa0ec 210%
    );
  }

  50% {
    background: linear-gradient(
      to bottom right,
      #8a8af4 -100%,
      #ffa0ec 0%,
      #8a8af4 100%,
      #ffa0ec 200%
    );
  }

  55% {
    background: linear-gradient(
      to bottom right,
      #8a8af4 -110%,
      #ffa0ec -10%,
      #8a8af4 90%,
      #ffa0ec 190%
    );
  }

  60% {
    background: linear-gradient(
      to bottom right,
      #8a8af4 -120%,
      #ffa0ec -20%,
      #8a8af4 80%,
      #ffa0ec 180%
    );
  }

  65% {
    background: linear-gradient(
      to bottom right,
      #8a8af4 -130%,
      #ffa0ec -30%,
      #8a8af4 70%,
      #ffa0ec 170%
    );
  }

  70% {
    background: linear-gradient(
      to bottom right,
      #8a8af4 -140%,
      #ffa0ec -40%,
      #8a8af4 60%,
      #ffa0ec 160%
    );
  }

  75% {
    background: linear-gradient(
      to bottom right,
      #8a8af4 -150%,
      #ffa0ec -50%,
      #8a8af4 50%,
      #ffa0ec 150%
    );
  }

  80% {
    background: linear-gradient(
      to bottom right,
      #8a8af4 -160%,
      #ffa0ec -60%,
      #8a8af4 40%,
      #ffa0ec 140%
    );
  }

  85% {
    background: linear-gradient(
      to bottom right,
      #8a8af4 -170%,
      #ffa0ec -70%,
      #8a8af4 30%,
      #ffa0ec 130%
    );
  }

  90% {
    background: linear-gradient(
      to bottom right,
      #8a8af4 -180%,
      #ffa0ec -80%,
      #8a8af4 20%,
      #ffa0ec 120%
    );
  }

  95% {
    background: linear-gradient(
      to bottom right,
      #8a8af4 -190%,
      #ffa0ec -90%,
      #8a8af4 10%,
      #ffa0ec 110%
    );
  }

  100% {
    background: linear-gradient(
      to bottom right,
      #8a8af4 -200%,
      #ffa0ec -100%,
      #8a8af4 0%,
      #ffa0ec 100%
    );
  }
}

.qrScann {
  display: flex;
  flex-grow: 1;
  justify-content: end;
  margin-right: 12px;
}
</style>

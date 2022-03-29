<template>
    <div class="Navigation" data-tauri-drag-region>
        <!-- <el-button type="primary" color="#C7415B" :icon="ArrowLeftBold" @click="back" circle></el-button>
        <el-button
            type="primary"
            color="#C7415B"
            :class="onLoad === true ? 'onLoad' : ''"
            :icon="onLoad === true ? Loading : Download"
            @click="goSeek"
            circle
        ></el-button>-->

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
            ></navigation-item>
        </div>

        <el-popover
            placement="bottom"
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
                <el-button
                    type="danger"
                    :icon="Cloudy"
                    @mouseover="UpdatePopover.show = true"
                    @mouseleave="UpdatePopover.show = false"
                    @click="install"
                    circle
                />
            </template>
        </el-popover>
    </div>
</template>

<script lang="ts">
import { defineComponent, ref, onMounted, inject } from 'vue';
import { useRouter } from 'vue-router';
import { getCurrent, WebviewWindow } from '@tauri-apps/api/window';
import { listen, emit } from '@tauri-apps/api/event';
import { ArrowLeftBold, Loading, Download, Cloudy } from '@element-plus/icons-vue';
import { checkUpdate, installUpdate } from '@tauri-apps/api/updater';
import { relaunch } from '@tauri-apps/api/process';
import { ElLoading } from 'element-plus';
import 'element-plus/es/components/loading/style/css'
export default defineComponent({
    name: "Navigation",
    props: [],
    setup(props, { emit }) {
        const router = useRouter();
        const back = () => {
            router.back();
        };

        let webview: any = null;

        const Updater: any = ref({})

        const UpdatePopover = ref({
            Loading: false,
            show: false
        });

        const nav = ref({
            choose: 0,
            list: [
                { name: "首页", path: "/SomvView" },
                { name: "检索", path: "/SomvFile" },
                { name: "首页", path: "/SomvView" }
            ] as any[]
        } as any);

        onMounted(() => {
            webview = new WebviewWindow('seek', {
                url: '/seek',
                title: '检索列表',
                width: 500,
                height: 700,
                center: true,
                visible: false,
                alwaysOnTop: false,
                skipTaskbar: true,
                resizable: false,
                decorations: true
            });
            eventSeekStatus();
            linstenUpdate();
        });

        //增加控制 是否自动检测版本更新
        const linstenUpdate = async () => {
            Updater.value = await checkUpdate();

            console.log(Updater.value)
        }

        const eventSeekStatus = async () => {
            !(async () => await listen('seek_status', (event: any) => {
                console.log(event.payload);
                onLoad.value = eval(event.payload.toLowerCase());
            }))()
        }

        const install = async () => {
            const loading = ElLoading.service({
                lock: true,
                text: '正在下载更新，下载完成后会自动更新',
            })
            await installUpdate();
            // install complete, restart app
            await relaunch();
        }

        // const linstenUpdate = async () => {
        //     emit("tauri://update");
        //     !(async () => await listen('tauri://update-available', (event) => {
        //         console.log(event);
        //     }))()
        // }

        const onLoad = ref(false);

        const goSeek = () => {
            webview.show();
            webview.unminimize();

            webview.setAlwaysOnTop(true);
            setTimeout(() => {
                webview.setAlwaysOnTop(false)
            }, 50);
        }
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
            nav
        };
    },
})
</script>

<style scoped lang="less">
.Navigation {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
    z-index: 99999;
    align-items: center;

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
</style>
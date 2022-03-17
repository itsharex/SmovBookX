<template>
    <div class="Navigation">
        <el-button type="primary" :icon="ArrowLeftBold" @click="back" circle></el-button>
        <el-button
            type="primary"
            :class="onLoad === true ? 'onLoad' : ''"
            :icon="onLoad === true ? Loading : Download"
            @click="goSeek"
            circle
        ></el-button>

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
            // linstenUpdate();
        });

        //增加控制 是否自动检测版本更新

        const eventSeekStatus = async () => {
            Updater.value = await checkUpdate();

            console.log(Updater.value)
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
            install
        };
    },
})
</script>

<style scoped lang="less">
.Navigation {
    width: 100%;
    height: 100%;
    box-shadow: var(--el-box-shadow-light);
    display: flex;
    align-items: center;
    * {
        margin-left: 20px;
    }
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
</style>
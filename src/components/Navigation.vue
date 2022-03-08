<template>
    <div class="Navigation">
        <el-button type="primary" :icon="ArrowLeftBold" @click="back" circle></el-button>
        <el-button
            type="primary"
            :class="onLoad === true ? 'onLoad' : ''"
            :icon="onLoad === true ? Loading : ArrowDownBold"
            @click="goSeek"
            circle
        ></el-button>
    </div>
</template>

<script lang="ts">
import { defineComponent, ref, onMounted, inject } from 'vue';
import { useRouter } from 'vue-router';
import { getCurrent, WebviewWindow } from '@tauri-apps/api/window';
import { listen } from '@tauri-apps/api/event';
import { ArrowLeftBold, Loading, ArrowDownBold } from '@element-plus/icons-vue';
export default defineComponent({
    name: "Navigation",
    props: [],
    setup(props, { emit }) {
        const router = useRouter();
        const back = () => {
            router.back();
        };

        let webview: any = null;

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
        });

        const eventSeekStatus = () => {
            !(async () => await listen('seek_status', (event: any) => {
                console.log(event.payload);
                onLoad.value =eval(event.payload.toLowerCase());   
            }))()
        }

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
            ArrowDownBold,
            onLoad,
            Loading
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
</style>
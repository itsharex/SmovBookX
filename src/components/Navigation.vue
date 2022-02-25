<template>
    <div class="Navigation">
        <el-button type="primary" :icon="ArrowLeftBold" @click="back" circle></el-button>
        <el-button
            type="primary"
            :class="onLoad ? 'onLoad' : ''"
            :icon="onLoad ? Loading : ArrowDownBold"
            @click="goSeek"
            circle
        ></el-button>
    </div>
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue';
import { useRouter } from 'vue-router';
import { getCurrent, WebviewWindow } from '@tauri-apps/api/window';
import { ArrowLeftBold, Loading, ArrowDownBold } from '@element-plus/icons-vue';
export default defineComponent({
    name: "Navigation",
    props: [],
    setup(props, { emit }) {
        const router = useRouter();
        const back = () => {
            router.back();
        };

        const onLoad = ref(false);

        const goSeek = () => {
            let webview = new WebviewWindow('test3', {
                url: '/test3',
                title: '软件设置',
                width: 360,
                height: 280,
                center: true,
                visible: false,
                alwaysOnTop: true,
                skipTaskbar: true,
                resizable: false,
            });
            
            webview.show();
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
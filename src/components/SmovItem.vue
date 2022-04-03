<template>
    <div class="smovItem" @click="openDetail()">
        <img
            :src="data.thumbs_img == '' ? nonePic : convertFileSrc(data.thumbs_img)"
            class="smovImg"
        />
        <p class="name">{{ data.name }}</p>

        <el-popover
            placement="bottom"
            :width="200"
            trigger="click"
            :content="data.title"
            :visible="false"
        >
            <template #reference>
                <div class="titleDiv">
                    <p class="title">
                        <span>{{ data.title }}</span>
                    </p>
                </div>
            </template>
        </el-popover>
    </div>
</template>

<script lang='ts'>
import { defineComponent, ref } from 'vue';
import { convertFileSrc, invoke } from '@tauri-apps/api/tauri';
import nonePic from "../assets/none_pic.png";
import mountDetail from './SmovDetail/SmovDetail'
import XEUtils from 'xe-utils';
import { getAll, WebviewWindow, availableMonitors } from '@tauri-apps/api/window';

import { title } from 'process';
import { join } from 'path/posix';
import { ElSteps } from 'element-plus';
export default defineComponent({
    name: 'SmovItem',
    props: ['data'],
    setup(props, { emit }) {

        const openDetail = async () => {

            const webview = new WebviewWindow(props.data.name, {
                url: '/SmovDetail/' + props.data.id,
                title: props.data.name,
                center: true,
                minHeight: 600,
                minWidth: 800,
                decorations: false,
                fileDropEnabled: false
            });

            invoke("set_focus", { label: props.data.name });

            webview.once('tauri://created', function () {
                invoke("set_style", { effect: "", label: props.data.name });
            })
            webview.once('tauri://close-requested', function (e) {
                console.log(e.windowLabel)
            })

        }
        return {
            convertFileSrc,
            nonePic,
            openDetail,

        };
    }
})

</script>
<style lang='less' scoped>
.smovImg {
    display: inline-block;
    height: 200px;
    max-width: 100%;
    object-fit: cover;
    border-radius: 6px;
}
.smovItem {
    height: 280px;
    width: 160px;
    border-radius: 6px;
    box-shadow: var(--el-box-shadow-light);
    margin: 10px;
    cursor: pointer;
    transition: all 0.1s;
}

.smovItem:hover {
    transform: scale(1.05);
}

.name {
    font-size: 16px;
    font-weight: 700;
    margin-top: 4px;
    margin-bottom: 4px;
}

.titleDiv {
    // width: 100%;
    max-width: 160px;
    display: flex;
    justify-content: center;
}

.title {
    width: 80%;
    display: -webkit-box;
    word-break: break-all;
    text-overflow: ellipsis;
    font-size: 13px;
    overflow: hidden;
    -webkit-box-orient: vertical;
    -webkit-line-clamp: 1;
    margin-top: 4px;
    margin-bottom: 4px;
}
</style>
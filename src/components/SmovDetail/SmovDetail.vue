<template>
    <div class="smovDetail">
        <el-dialog v-model="dialogVisible" width="80%" :before-close="close" :lock-scroll="true">
            <!-- <Artplayer @get-instance="getInstance" :option="option" :style="style" /> -->
            <div class="imgDiv">
                <img
                    class="mainImg"
                    :src="data.main_img == '' ? nonePic : convertFileSrc(data.main_img)"
                />
            </div>

            <p class="note">WebView2存在视频内存泄漏的问题 ， 等待后续更新 增加软件内观看的功能 ,当前内置播放体验差 建议不要用</p>
            <template #footer>
                <el-button type="info" color="#C7415B" @click="close">关闭</el-button>

                <el-button type="info" color="#C7415B" @click="toOpen">由本地播放器打开</el-button>
            </template>
        </el-dialog>
    </div>
</template>

<script lang='ts'>
import { shell } from '@tauri-apps/api';
import { join } from '@tauri-apps/api/path';
import { convertFileSrc } from '@tauri-apps/api/tauri';
import { defineComponent } from 'vue';
import nonePic from './NoneImages.png'

export default defineComponent({
    props: {
        data: {
            type: Object,
            default: {}
        },
        close: {
            type: Function as any
        },
    },
    setup(props) {
        const option = {
            url: convertFileSrc(props.data.path + '//' + props.data.realname + '.' + props.data.extension),
            fullscreen: true,
            fullscreenWeb: true,
            theme: '#ffad00',
        };
        const style = {
            width: "600px",
            height: "400px",
            margin: "60px auto 0",
        };

        const getInstance = (art: any) => {
            console.log(art);
        };

        const toOpen = async () => {
            shell.open(await join(props.data.path, props.data.realname + '.' + props.data.extension));
        }

        return {
            option,
            style,
            getInstance,
            convertFileSrc,
            toOpen,
            nonePic,
            dialogVisible: true
        };
    }
})

</script>
<style lang='less' scoped>
.note {
    font-weight: 700;
}
.imgDiv{
    width: 100%;
    height: 60%;
}

.mainImg {
    display: inline-block;
    max-width: 80%;
    max-height: 100%;
    object-fit: cover;
    border-radius: 6px;
}

.smovDetail {
    width: 60%;
    height: 50%;
    z-index: 998;
    position: absolute;
    top: 0;
}
</style>

<style>
.domSmovDetail {
    width: 100%;
    height: 100%;
}
</style>
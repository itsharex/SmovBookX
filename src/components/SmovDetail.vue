<template>
    <div>
        <!-- tauri不能打开本地文件  这个播放器的体验非常差 不知道别人用不用 反正我不用  -->
        <Artplayer @get-instance="getInstance" :option="option" :style="style" />
        
        <el-button type="info" color="#C7415B" @click="toOpen">由本地播放器打开</el-button>
    </div>
</template>

<script lang='ts'>
import { shell } from '@tauri-apps/api';
import { convertFileSrc } from '@tauri-apps/api/tauri';
import { defineComponent, onMounted, ref } from 'vue';

export default defineComponent({
    name: 'test1',
    props: [],

    setup(props, { emit }) {
        const option = {
            url: convertFileSrc("C:\\Users\\Leri\\Desktop\\4K Istanbul City.webm"),
            fullscreen: true,
            fullscreenWeb: true,
            theme: '#ffad00',
        } ;
        const style = {
            width: "600px",
            height: "400px",
            margin: "60px auto 0",
        };

        const getInstance = (art: any) => {
            console.log(art);
        };

        const toOpen = async () => {
           shell.open("C:\\Users\\Leri\\Desktop\\4K Istanbul City.webm");
        }

        return {
            option,
            style,
            getInstance,
            convertFileSrc,
            toOpen
        };
    }
})

</script>
<style lang='less' scoped>
</style>
<template>
    <div>
        <Artplayer @get-instance="getInstance" :option="option" :style="style" />
    </div>
</template>

<script lang='ts'>
import { defineComponent, onMounted, ref } from 'vue';
import Artplayer from 'artplayer/examples/vue/Artplayer.vue'

export default defineComponent({
    name: 'test1',
    props: [],

    setup(props, { emit }) {
        const option = {
            url: "https://artplayer.org/assets/sample/video.mp4",
        };
        const style = {
            width: "600px",
            height: "400px",
            margin: "60px auto 0",
        };

        const getInstance = (art: any) => {
            console.log(art);
        };

        return {
            option,
            style,
            getInstance

        };
    }
})

</script>
<style lang='less' scoped>
</style>
<template>
    <div class="ImgTray">
        <div v-for="(item, index) in Imgs" :key="index">
            <img :src="item" alt="">
        </div>
    </div>
</template>

<script lang='ts'>
import { defineComponent, ref } from 'vue';
export default defineComponent({
    name: 'ImgTray',
    props: {
        Imgs: {
            type: Array as any ,
        }
    },
    setup(props, { emit }) {
        return {
        };
    }
})

</script>
<style lang='less' scoped>
.ImgTray {
    width: 100%;
}
</style>
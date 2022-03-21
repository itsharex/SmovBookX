<template>
    <div class="smovItem">
        <img :src="convertFileSrc(data.thumbs_img)" class="smovImg" />
        <p class="name">{{data.name}}</p>
        <p class=""></p>
    </div>
</template>

<script lang='ts'>
import { defineComponent, ref } from 'vue';
import { convertFileSrc } from '@tauri-apps/api/tauri';
export default defineComponent({
    name: 'SmovItem',
    props: ['data'],
    setup(props, { emit }) {
        return {
            convertFileSrc
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
    height: 250px;
    border-radius: 6px;
    box-shadow: var(--el-box-shadow-light);
    margin: 10px;
}

.name{
    font-size: 13px;
    font-weight: 600;
}
</style>
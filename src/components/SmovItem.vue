<template>
    <div class="smovItem">
        <img
            :src="data.thumbs_img == '' ? nonePic : convertFileSrc(data.thumbs_img)"
            class="smovImg"
        />
        <p class="name">{{ data.name }}</p>

        <el-popover placement="bottom" :width="200" trigger="hover" :content="data.title">
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
import { convertFileSrc } from '@tauri-apps/api/tauri';
import nonePic from "../assets/none_pic.png";
export default defineComponent({
    name: 'SmovItem',
    props: ['data'],
    setup(props, { emit }) {
        return {
            convertFileSrc,
            nonePic
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
    width: 170px;
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
    width: 100%;
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
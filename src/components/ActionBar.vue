<template>
    <div class="ActionBar">
        <slot></slot>
        <div class="WindowAction" >
            <el-icon :size="17" class="WindowButton" @click="window.minimize()">
                <semi-select />
            </el-icon>

            <el-icon :size="17" class="WindowButton" @click="window.toggleMaximize()">
                <promotion />
            </el-icon>

            <el-icon :size="17" class="WindowButton" @click="window.hide()">
                <close-bold />
            </el-icon>

                        <!-- <smov-ico class="WindowButton" :name="'browser_window'" />
            -->
        </div>
    </div>
</template>

<script lang='ts' setup>
import { CloseBold, SemiSelect, Promotion } from '@element-plus/icons-vue';
import { getCurrent } from '@tauri-apps/api/window';

let WindowMaxSize = false;

const window = getCurrent();


</script>
<style lang='less' scoped>
.ActionBar {
    width: 100%;
    height: 100%;
    display: flex;
    justify-content: right;
}

.WindowButton {
    width: 1em;
    height: 1em;
    margin-left: 1.1em;
}

.WindowAction {
    height: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
   
}
</style>
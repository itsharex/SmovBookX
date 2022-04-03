<template>
    <div class="ActionBar" data-tauri-drag-region>
        <slot></slot>
        <div class="WindowAction">
            <el-icon
                v-show="top && !AlwaysOnTop"
                class="WindowButton"
                :size="17"
                @click="ChangeAlwaysOnTop()"
            >
                <circle-check />
            </el-icon>
            <el-icon
                v-show="top && AlwaysOnTop"
                class="WindowButton"
                :size="17"
                @click="ChangeAlwaysOnTop()"
            >
                <circle-check-filled />
            </el-icon>

            <el-icon :size="17" v-if="minImize" class="WindowButton" @click="window.minimize()">
                <semi-select />
            </el-icon>

            <el-icon
                :size="17"
                v-if="imize"
                class="WindowButton"
                @click="window.toggleMaximize()"
            >
                <promotion />
            </el-icon>

            <el-icon :size="17" class="WindowButton" @click="once ? window.close() : window.hide()">
                <close-bold />
            </el-icon>

            <!-- <smov-ico class="WindowButton" :name="'browser_window'" />
            -->
        </div>
    </div>
</template>

<script lang='ts' setup>
import { CloseBold, SemiSelect, Promotion, Place, CircleCheck, CircleCheckFilled } from '@element-plus/icons-vue';
import { getCurrent } from '@tauri-apps/api/window';

const window = getCurrent();

const AlwaysOnTop = ref(false);

const ChangeAlwaysOnTop = () => {
    AlwaysOnTop.value = !AlwaysOnTop.value;
    window.setAlwaysOnTop(AlwaysOnTop.value);
}

const props = defineProps({
    imize: {
        type: Boolean,
        default: true
    },
    close: {
        type: Boolean,
        default: true
    },
    minImize: {
        type: Boolean,
        default: true
    },
    top: {
        type: Boolean,
        default: false
    },
    once: {
        type: Boolean,
        default: false
    }
})


</script>
<style lang='less' scoped>
.ActionBar {
    width: 100%;
    height: 100%;
    display: flex;
    justify-content: right;
    align-items: center;
    z-index: 9999;
    & > * {
        cursor: pointer;
    }
}

.WindowButton {
    width: 2.8rem;
    height: 100%;
}

.WindowButton:hover {
    background-color: rgba(0, 0, 0, 0.144);
}

.WindowAction {
    height: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
}
</style>
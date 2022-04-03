<template>
    <div class="FileInput">
        <el-input :placeholder="value" v-model="path">
            <template #suffix>
                <el-icon :size="17" class="FileIco el-input__icon" @click="openSelect">
                    <folder-add />
                </el-icon>
            </template>
        </el-input>

        <el-button
            class="check"
            type="info"
            plain
            :icon="Check"
            @click="check"
            v-show="path != ''"
        />
    </div>
</template>

<script lang='ts' setup>
import { FolderAdd, Check } from '@element-plus/icons-vue';
import { open } from '@tauri-apps/api/dialog'

const em = defineEmits(['path', 'check'])

const path = ref('' as any);

const openSelect = () => {
    const option = {
        title: props.title,
        multiple: props.multiple,
        directory: props.directory,
    }
    open(option).then(res => {
        if (res) {
            path.value = res
        }
    })

};

const clean = () =>{
    path.value = ''
}

const check = () => {
    em("check", path.value);
}

const props = defineProps({
    title: {
        type: String,
        default: "选择文件夹"
    },
    multiple: {
        type: Boolean,
        default: false
    },
    directory: {
        type: Boolean,
        default: true
    },
    value: {
        type: String,
        default: ""
    }
})

defineExpose({
  clean
})

</script>
<style lang='less' scoped>
.FileIco {
    cursor: pointer;
}

.FileInput {
    display: flex;
    flex-direction: row;
    .el-input {
        width: 80%;
    }
}

.check {
    margin-left: 2rem;
}
</style>
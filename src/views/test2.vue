<template>
    <div>
        <el-button v-if="show" type="primary" @click="show = !show">Primary</el-button>
        <el-button v-if="show" type="success" @click="show = !show">Success</el-button>
    </div>
</template>

<script lang='ts' setup>
let show = false;
</script>
<style lang='less' scoped>
</style>
<template>
    <div></div>
</template>

<script lang='ts'>
import { invoke } from '@tauri-apps/api';
import { defineComponent, onMounted, ref } from 'vue';
export default defineComponent({
    name: 'SmovView',
    props: [],
    setup(props, { emit }) {

        onMounted(() => {
            getSmov();
        })

        const getSmov = () => {
            invoke("get_all_smov").then((res: any) => {
                 console.log(res)
            }).finally(() => {

            })
        }
        return {
        };
    }
})

</script>
<style lang='less' scoped>
</style>
<template>
    <div>
        <div class="smovList">
            <SmovItem v-for="(item, index) in smovList" :key="index" :data="item" />
        </div>
    </div>
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

        const smovList = ref([] as any[]);

        const getSmov = () => {
            invoke("get_all_smov").then((res: any) => {
                if (res.code = 200) {
                    smovList.value = res.data;
                }
            }).finally(() => {

            })
        }
        return {
            smovList
        };
    }
})

</script>
<style lang='less' scoped>
.smovList {
    display: flex;
    flex-wrap: wrap;
    justify-content: center;
}
</style>
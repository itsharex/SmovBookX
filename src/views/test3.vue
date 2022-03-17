<template>
    <div>{{time}}</div>
</template>

<script lang='ts'>
import { defineComponent, ref, onMounted } from 'vue';
export default defineComponent({
    name: 'test3',
    props: [],
    setup(props, { emit }) {

        let time = ref(0);

        onMounted(() =>{
           setInterval(() => {
                time.value = time.value + 1;
            }, 1000)
        })

        return {
            time
        };
    }
})

</script>
<style lang='less' scoped>
</style>
<template>
    <div ref="artRef"></div>
</template>

<script lang='ts'>
import { defineComponent, nextTick, onMounted, ref } from 'vue';
import Artplayer from 'artplayer'

export default defineComponent({
    name: 'Artplayer',
    props: {
        option: {
            type: Object,
            required: true,
        }
    },
    setup(props, { emit }) {

        const artRef = ref({} as any);

        onMounted(() => {
            const instance = new Artplayer({ ...props.option, container: artRef.value });
            nextTick(() => {
                emit('get-instance', instance);
            });
        });



        return {
            artRef

        };
    }
})

</script>
<style lang='less' scoped>
</style>



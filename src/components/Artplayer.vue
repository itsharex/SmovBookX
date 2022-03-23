<template>
    <div ref="artRef"></div>
</template>

<script>
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

        const artRef = ref();

        let instance = {};

        const fullscreen = (args) => {
            if (args === true) {
                getCurrent().fullscreen = true;
            }
        }

        const eventFull = async () => {
            !(async () => await instance.on('fullscreen', (event) => {
                console.log(event);
            }))()
        }

        onMounted(() => {
            instance = new Artplayer({ ...props.option, container: artRef.value });
            nextTick(() => {
                // emit('get-instance', instance);
            });
            eventFull();
        });



        return {
            artRef

        };
    }
})

</script>
<style lang='less' scoped>
</style>



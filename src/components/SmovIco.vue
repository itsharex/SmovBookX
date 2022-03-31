<script setup lang="ts">
import { computed } from '@vue/reactivity';


const props = defineProps({
    name: {
        type: String,
        required: true
    },
    color: {
        type: String,
        default: ''
    }
})

const iconName = computed(() => `#icon-${props.name}`);
const svgClass = computed(() => {
    if (props.name) {
        return `svg-icon icon-${props.name}`
    }
    return `svg-icon`
})

</script>
 
<template>
    <i>
        <svg :class="svgClass" class="el_icon" v-bind="$attrs" :style="{ color: color }">
            <use :xlink:href="iconName" />
        </svg>
    </i>
</template>
 
<style lang="less" scoped>
.svg-icon {
    width: 1em;
    height: 1em;
    fill: currentColor;
    overflow: hidden;
}
</style>
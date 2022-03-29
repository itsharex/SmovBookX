<template>
    <div class="NavItem" :class="choose ? 'NavItemChoose' : ''">{{ name }}</div>
</template>

<script lang='ts' setup>
import { useRouter } from 'vue-router';
import { defineComponent, ref, defineProps } from 'vue';
const router = useRouter();
 

// expects props options
const props = defineProps({
    name: String,
    choose: String,
    path: String
})

</script>
<style lang='less' scoped>
.NavItem {
    border-radius: 4px;
}

.NavItemChoose {
    background-color: rgba(194, 194, 194, 0.548);
}
</style>
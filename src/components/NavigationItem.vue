<template>
    <div class="NavItem" :class="choose ? 'NavItemChoose' : ''" @click="router_go"><span>{{ name }}</span></div>
</template>

<script lang='ts' setup>
import { useRouter } from 'vue-router';
import { defineComponent, ref, defineProps } from 'vue';
const router = useRouter();

const router_go = () => {
    console.log("被调用了")
    router.push({
        path: props.path,
    })
}

const props = defineProps({
    name: String as any,
    choose: String as any,
    path: String as any
})

</script>
<style lang='less' scoped>
.NavItem {
    border-radius: 4px;
    width: 100%;
    height: 30px;
    cursor: pointer;
    display: flex;
    justify-content: left;
    align-items: center;
    font-size: 1.1rem;
    font-weight: 500;
    span{
        margin-left: 2rem;
    }
}

.NavItem:hover {
    background-color: rgba(216, 216, 216, 0.63);
}

.NavItemChoose {
    background-color: rgba(194, 194, 194, 0.548);
}
</style>
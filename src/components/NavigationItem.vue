<template>
    <div
        class="NavItem"
        :class="choose === index ? 'NavItemChoose' : ''"
        @click="router_go"
        v-if="show"
    >
        <div class="ChooseItem" :class="choose === index ? 'NavItemChooseT' : ''"></div>
 
        <span>{{ name }}</span>
    </div>
</template>

<script lang='ts' setup>
import { defineCustomElement } from '@vue/runtime-dom';
import { useRouter } from 'vue-router';

const router = useRouter();

const router_go = () => {
    changeChoose();
    router.push({
        path: props.path,
    })
}

const em = defineEmits(['changeChoose'])

const changeChoose = () => {
    em("changeChoose", props.index)
}

const props = defineProps({
    name: String as any,
    choose: Number as any,
    path: String as any,
    index: Number as any,
    show: Boolean as any,
    ico: {} as any
})

</script>
<style lang='less' scoped>
.NavItem {
    height: 100%;
    border-radius: 4px;
    height: 30px;
    cursor: pointer;
    display: flex;
    justify-content: left;
    align-items: center;
    font-size: 0.9rem;
    font-weight: 600;
    margin-bottom: 1px;
    padding: 2px;
    span {
        margin-left: 0.5rem;
    }
}

.NavItem:hover {
    background-color: rgba(224, 224, 224, 0.226);
}

.NavItemChoose {
    background-color: rgba(224, 224, 224, 0.548);
}
.ChooseItem {
    width: 10px;
    height: 100%;
    border-radius: 4px;
}

.NavItemChooseT {
    background: rgba(0, 128, 0, 0.322);
}
</style>
<template>
  <div class="ActionBarButton">
    <div class="ActionBarIco">
      <slot></slot>
    </div>
  </div>
</template>

<script lang="ts" setup>
const props = defineProps({
  close: {
    type: Boolean,
    default: true,
  },
});
</script>

<style lang="less" scoped>
.ActionBarButton {
  width: 2.5rem;
  height: 2.1rem;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-content: center;
  cursor: pointer;
}

.ActionBarButton:hover {
  background-color: rgba(0, 0, 0, 0.144);
}

.ActionBarIco {
  flex-grow: 0;
  flex-shrink: 0;
}
</style>
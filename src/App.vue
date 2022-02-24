<template>
  <div class="app">
    <el-button type="primary" :icon="ArrowLeftBold" @click="back" circle></el-button>
    
    <router-view />
    <Log />
  </div>
</template>

<script lang="ts">
import { ArrowLeftBold } from '@element-plus/icons-vue';
import { defineComponent, ref, onMounted, inject, watch, computed } from "vue";
import { useRouter, useRoute } from 'vue-router';
import Log from './components/Log.vue';

export default defineComponent({
  components: { Log },
  setup() {
    const router = useRouter();
    const back = () => {
      router.back();
    };
    return {
      ArrowLeftBold,
      back
    };
  },
});

</script>

<style lang="less">
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  padding: 0%;
}

#nav {
  padding: 30px;

  a {
    font-weight: bold;
    color: #2c3e50;

    &.router-link-exact-active {
      color: #42b983;
    }
  }
}

// body::-webkit-scrollbar {
//   width: 0 !important;
// }

// body::-webkit-scrollbar {
//   width: 0 !important;
//   height: 0;
// }
</style>




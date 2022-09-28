import { defineConfig, loadEnv } from "vite";
import vue from "@vitejs/plugin-vue";
import AutoImport from 'unplugin-auto-import/vite'
import Components from 'unplugin-vue-components/vite'
import { ElementPlusResolver } from 'unplugin-vue-components/resolvers'
import OptimizationPersist from 'vite-plugin-optimize-persist'
import PkgConfig from 'vite-plugin-package-config'
// import { svgBuilder } from "./svgBuilder";

export default defineConfig({
  plugins: [
    vue(),
    AutoImport({
      dts: 'src/auto-imports.d.ts',
      imports: ["vue", "vue-router"],
      resolvers: [ElementPlusResolver()],
    }),
    Components({
      extensions: ['vue', 'md'],
      // ,"node_modules/artplayer/examples/vue"
      dirs: ["src/components"],
      deep: true,
      include: [/\.vue$/, /\.vue\?vue/, /\.md$/],
      resolvers: [
        ElementPlusResolver(),
      ],
      dts: 'src/components.d.ts',
    }),
    PkgConfig(),
    OptimizationPersist()
    // svgBuilder('./src/assets/svg/'),
  ],
  server: {
    hmr: { overlay: false },
  },
  resolve: {
    alias: { vue: "vue/dist/vue.esm-bundler.js" },
  },
  define: {
    'process.env': process.env
  }

});

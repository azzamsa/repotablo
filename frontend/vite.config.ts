import { URL, fileURLToPath } from 'node:url'
import vue from '@vitejs/plugin-vue'
import AutoImport from 'unplugin-auto-import/vite'
import IconsResolver from 'unplugin-icons/resolver'
import Icons from 'unplugin-icons/vite'
import Components from 'unplugin-vue-components/vite'
import { defineConfig } from 'vite'
import Pages from 'vite-plugin-pages'
import Layouts from 'vite-plugin-vue-layouts'

export default () => {
  // https://vitejs.dev/config/
  return defineConfig({
    plugins: [
      vue({
        reactivityTransform: true,
      }),

      // https://github.com/hannoeru/vite-plugin-pages
      Pages(),

      // https://github.com/JohnCampionJr/vite-plugin-vue-layouts
      Layouts(),

      // https://github.com/antfu/unplugin-vue-components
      Components({
        resolvers: [
          // Allow auto import `unplugin/icons`
          IconsResolver(),
          // Import `Head` from  '@vueuse/head'
          (componentName) => {
            // where `componentName` is always CapitalCase
            if (componentName === 'Head')
              return { name: 'Head', from: '@vueuse/head' }
          },
        ],
      }),

      // https://github.com/antfu/unplugin-icons/
      Icons(),

      // https://github.com/antfu/unplugin-auto-import
      AutoImport({
        imports: [
          'vue',
          'vue-router',
          'vue/macros',
          // https://github.com/vueuse/head
          '@vueuse/head',
          '@vueuse/core',
        ],
        dts: 'src/auto-imports.d.ts',
        dirs: ['src/stores'],
      }),
    ],

    // https://github.com/vitest-dev/vitest
    test: {
      include: ['tests/**/*.test.ts'],
      environment: 'jsdom',
      deps: {
        inline: ['@vue', '@vueuse'],
      },
    },

    resolve: {
      alias: {
        '@': fileURLToPath(new URL('./src', import.meta.url)),
      },
    },
  })
}

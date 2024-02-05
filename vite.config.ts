import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import topLevelAwait from "vite-plugin-top-level-await";
import { viteSingleFile } from "vite-plugin-singlefile"

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    svelte(),
    topLevelAwait(),
    viteSingleFile(),
  ],
  build: {
    assetsDir: ".",
  },
})

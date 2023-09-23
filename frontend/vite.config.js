import { resolve } from 'path';
import { defineConfig } from 'vite';
import { viteSingleFile } from "vite-plugin-singlefile"

export default defineConfig({
  plugins: [viteSingleFile()],
  resolve: {
    preserveSymlinks: true, // this is the fix!
  },
  build: {
    minify: false,
    cssCodeSplit: false,
    lib: {
      // Could also be a dictionary or array of multiple entry points
      entry: resolve(__dirname, 'src/main.ts'),
      name: 'TypstBook',
      // the proper extensions will be added
      fileName: 'book',
    },
  },
});

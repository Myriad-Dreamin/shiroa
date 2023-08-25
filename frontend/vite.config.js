import { resolve } from 'path';
import { defineConfig } from 'vite';

export default defineConfig({
  resolve: {
    preserveSymlinks: true, // this is the fix!
  },
  build: {
    minify: false,
    lib: {
      // Could also be a dictionary or array of multiple entry points
      entry: resolve(__dirname, 'src/main.ts'),
      name: 'TypstBook',
      // the proper extensions will be added
      fileName: 'book',
    },
  },
});

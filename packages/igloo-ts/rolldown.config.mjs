import { defineConfig } from 'rolldown';

export default defineConfig({
  input: 'src/index.ts',
  output: {
    dir: 'dist',
    format: 'esm',
  },
  // The iced:app/* WIT interfaces are provided by jco componentize, not resolvable on disk.
  external: /^iced:/,
});

import {defineConfig, presetUno} from 'unocss';

export default defineConfig({
  content: {
    filesystem: ['**/*.{rs}']
  },
  presets: [
    presetUno()
  ]
});

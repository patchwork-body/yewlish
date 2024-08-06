import {defineConfig, presetUno} from 'unocss';

export default defineConfig({
  content: {
    filesystem: ['**/*.{rs}']
  },
  presets: [
    presetUno(),
  ],
  theme: {
    animation: {
      keyframes: {
        'fade-in': `{
          from { opacity: 0; }
          to { opacity: 1; }
        }`,
        'fade-out': `{
          from { opacity: 1; }
          to { opacity: 0; }
        }`,
      },
      durations: {
        'fade-in': '0.3s',
        'fade-out': '0.3s',
      },
      timingFns: {
        'fade-in': 'ease-in-out',
        'fade-out': 'ease-in-out',
      },
      counts: {
        'fade-in': 1,
        'fade-out': 1,
      },
    }
  },
  variants: [
    (matcher) => {
      if (!matcher.startsWith('data-[')) return matcher
      const match = matcher.match(/^data-\[(.+?)\]:(.+)/)
      if (!match) return matcher

      return {
        matcher: match[2],
        selector: s => `[data-${match[1]}] ${s}`,
      }
    },

    (matcher) => {
      if (!matcher.startsWith('aria-[')) return matcher
      const match = matcher.match(/^aria-\[(.+?)\]:(.+)/)
      if (!match) return matcher

      return {
        matcher: match[2],
        selector: s => `[aria-${match[1]}] ${s}`,
      }
    },
  ]
});

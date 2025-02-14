import type {Config} from 'tailwindcss';

import kampsyUI from 'kampsy-ui/preset';

const config: Config = {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  presets: [kampsyUI],
};

export default config;
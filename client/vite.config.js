import { sveltekit } from '@sveltejs/kit/vite';
import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';
import { defineConfig } from 'vite';
import tailwindcss from '@tailwindcss/vite';

/** @type {import('@sveltejs/kit/vite').Config} */
export const svelteKitConfig = {
	// Consult https://svelte.dev/docs/kit/integrations
	// for more information about preprocessors
	preprocess: vitePreprocess(),

	// Using static adapter for Docker deployment
	adapter: adapter({
		// Default options work for most cases
		pages: 'build',
		assets: 'build',
		fallback: 'index.html',
		precompress: false,
		strict: true
	})
};

export default defineConfig({
	plugins: [tailwindcss(), sveltekit(svelteKitConfig)]
});

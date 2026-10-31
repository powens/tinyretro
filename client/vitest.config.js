import { defineConfig } from "vitest/config";
import { sveltekit } from "@sveltejs/kit/vite";
import { svelteTesting } from "@testing-library/svelte/vite";
import { svelteKitConfig } from "./vite.config.js";

export default defineConfig({
  plugins: [sveltekit(svelteKitConfig), svelteTesting()],
  test: {
    environment: "happy-dom",
    globals: true,
    setupFiles: ["./vitest-setup.js"],
  },
});

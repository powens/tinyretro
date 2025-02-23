import globals from "globals";
import pluginJs from "@eslint/js";
import tseslint from "typescript-eslint";
import sveltePlugin from "eslint-plugin-svelte";
import svelteParser from "svelte-eslint-parser";

/** @type {import('eslint').Linter.Config[]} */
export default [
  ...sveltePlugin.configs["flat/recommended"],

  { files: ["**/*.{js,mjs,cjs,ts}"] },
  { languageOptions: { globals: globals.browser } },
  pluginJs.configs.recommended,
  ...tseslint.configs.recommended,
  {
    files: ["**/*.svelte", "*.svelte"],
    languageOptions: {
      parser: svelteParser,
      parserOptions: {
        parser: {
          ts: "@typescript-eslint/parser",
        },
        extraFileExtensions: [".svelte"],
      },
    }
  },
  {
    files: ["**/*.svelte.ts", "*.svelte.ts"],
    languageOptions: {
      parser: svelteParser,
      parserOptions: {
        parser: tseslint.parser,
      },
    }
  },
];

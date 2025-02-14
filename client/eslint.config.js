import eslintPluginSvelte from 'eslint-plugin-svelte';
export default [
  // add more generic rule sets here, such as:
  // js.configs.recommended,
  ...eslintPluginSvelte.configs.recommended,
  {
    // files: ["**/*.ts", "**/*.tsx", "**/*.svelte"],
    rules: {
      // override/add rules settings here, such as:
      // 'svelte/rule-name': 'error'
    }
  }
];
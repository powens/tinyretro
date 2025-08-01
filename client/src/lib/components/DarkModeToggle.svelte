<script lang="ts">
  import { Sun, Moon } from "lucide-svelte";
  import { Button } from "$lib/components/ui/button";

  let {
    isDarkMode = $bindable(),
    variant = "ghost",
    size = "icon",
    persistToStorage = false,
  }: {
    isDarkMode: boolean;
    variant?:
      | "default"
      | "destructive"
      | "outline"
      | "secondary"
      | "ghost"
      | "link";
    size?: "default" | "sm" | "lg" | "icon";
    persistToStorage?: boolean;
  } = $props();

  function toggleDarkMode() {
    isDarkMode = !isDarkMode;
    if (isDarkMode) {
      document.documentElement.classList.add("dark");
    } else {
      document.documentElement.classList.remove("dark");
    }

    // Optionally persist to localStorage
    if (persistToStorage && typeof window !== "undefined") {
      localStorage.setItem("theme", isDarkMode ? "dark" : "light");
    }
  }
</script>

<Button
  {variant}
  {size}
  onclick={toggleDarkMode}
  title={isDarkMode ? "Switch to light mode" : "Switch to dark mode"}
>
  {#if isDarkMode}
    <Sun class="w-5 h-5" />
  {:else}
    <Moon class="w-5 h-5" />
  {/if}
</Button>

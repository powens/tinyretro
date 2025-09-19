<script lang="ts">
  import { Plus } from "lucide-svelte";
  import { Button } from "$lib/components/ui/button";
  import { Badge } from "$lib/components/ui/badge";
  import DarkModeToggle from "./DarkModeToggle.svelte";

  let {
    isDarkMode = $bindable(),
    onAddCategory,
    sprintNumber = "24",
  }: {
    isDarkMode: boolean;
    onAddCategory: () => void;
    sprintNumber?: string;
  } = $props();
</script>

<header
  class="bg-white dark:bg-gray-800 shadow-sm border-b border-gray-200 dark:border-gray-700"
>
  <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
    <div class="flex justify-between items-center h-16">
      <div class="flex items-center">
        <h1 class="text-2xl font-bold text-gray-900 dark:text-white">
          tinyretro
        </h1>
        <Badge variant="secondary" class="ml-3">Sprint {sprintNumber}</Badge>
      </div>

      <div class="flex items-center space-x-4">
        <!-- Add Category Button -->
        <Button onclick={onAddCategory} class="gap-1">
          <Plus class="w-4 h-4" />
          Add Category
        </Button>

        <!-- Dark Mode Toggle -->
        <DarkModeToggle bind:isDarkMode persistToStorage={true} />
      </div>
    </div>
  </div>
</header>

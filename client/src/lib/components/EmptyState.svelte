<script lang="ts">
  import { Plus } from "lucide-svelte";
  import { Button } from "$lib/components/ui/button";

  let { onAddCategory }: { onAddCategory: () => void } = $props();
</script>

<div class="text-center py-12">
  <div class="max-w-md mx-auto">
    <div
      class="w-16 h-16 mx-auto mb-4 bg-gray-100 dark:bg-gray-800 rounded-full flex items-center justify-center"
    >
      <Plus class="w-8 h-8 text-gray-400" />
    </div>
    <h3 class="text-lg font-medium text-gray-900 dark:text-white mb-2">
      No categories yet
    </h3>
    <p class="text-gray-500 dark:text-gray-400 mb-6">
      Get started by creating your first category for the retrospective.
    </p>
    <Button onclick={onAddCategory} class="gap-2">
      <Plus class="w-4 h-4" />
      Add First Category
    </Button>
  </div>
</div>

<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import * as Dialog from "$lib/components/ui/dialog";

  let {
    open = $bindable(),
    categoryTitle = $bindable(""),
    onAddCategory,
    onCancel,
  }: {
    open: boolean;
    categoryTitle: string;
    onAddCategory: () => void;
    onCancel: () => void;
  } = $props();

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Enter") {
      onAddCategory();
    }
  }
</script>

<Dialog.Root bind:open>
  <Dialog.Content class="sm:max-w-md">
    <Dialog.Header>
      <Dialog.Title>Add New Category</Dialog.Title>
    </Dialog.Header>
    <div class="space-y-4">
      <Input
        bind:value={categoryTitle}
        placeholder="Category title"
        onkeydown={handleKeydown}
      />
      <Dialog.Footer>
        <Button variant="outline" onclick={onCancel}>Cancel</Button>
        <Button onclick={onAddCategory}>Add Category</Button>
      </Dialog.Footer>
    </div>
  </Dialog.Content>
</Dialog.Root>

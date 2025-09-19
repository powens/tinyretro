<script lang="ts">
  import { Card, CardContent, CardHeader } from "$lib/components/ui/card";
  import { Badge } from "$lib/components/ui/badge";
  import RetroItemCard from "./RetroItemCard.svelte";
  import AddItemForm from "./AddItemForm.svelte";
  import { dndzone, type DndEvent } from "svelte-dnd-action";

  import { flip } from "svelte/animate";

  type Item = {
    id: string;
    content: string;
    votes: number;
    sortOrder: number;
  };

  type Category = {
    id: string;
    title: string;
    theme: string;
    color: string;
    items: Item[];
  };

  let {
    category,
    editingItem = $bindable(),
    editingContent = $bindable(),
    onMoveItem,
    onReorderItems,
    onAddItem,
    onVoteItem,
    onDeleteItem,
    onStartEdit,
    onSaveEdit,
    onCancelEdit,
  }: {
    category: Category;
    editingItem: string | null;
    editingContent: string;
    onMoveItem: (
      itemId: string,
      fromLaneId: string,
      toLaneId: string,
      newIndex: number
    ) => void;
    onReorderItems: (laneId: string, items: Item[]) => void;
    onAddItem: (laneId: string, content: string) => void;
    onVoteItem: (laneId: string, itemId: string) => void;
    onDeleteItem: (laneId: string, itemId: string) => void;
    onStartEdit: (item: Item) => void;
    onSaveEdit: (laneId: string, itemId: string) => void;
    onCancelEdit: () => void;
  } = $props();

  let items = $state([...category.items]);
  let flipDurationMs = 300;

  function handleConsider(e: CustomEvent<DndEvent<Item>>) {
    console.debug("onConsider", e);
    items = e.detail.items;
  }

  function handleFinalize(e: CustomEvent<DndEvent<Item>>) {
    console.debug("onFinalize", e);
    items = e.detail.items;

    // Always call the reorder function when items are finalized
    onReorderItems(category.id, items);
  }
</script>

<Card class="overflow-hidden">
  <!-- Category Header -->
  <CardHeader class="pb-4">
    <div class="flex items-center">
      <div class="w-3 h-3 rounded-full {category.color} mr-3"></div>
      <h2 class="text-lg font-semibold">
        {category.title}
      </h2>
      <Badge variant="secondary" class="ml-auto">
        {category.items.length} items
      </Badge>
    </div>
  </CardHeader>

  <!-- Items with Drag and Drop -->
  <div
    class="space-y-3 min-h-[100px]"
    use:dndzone={{ items, flipDurationMs }}
    onconsider={handleConsider}
    onfinalize={handleFinalize}
  >
    <CardContent class="p-4 max-h-96 overflow-y-auto">
      {#each items as item (item.id)}
        <div animate:flip={{ duration: flipDurationMs }}>
          <RetroItemCard
            {item}
            laneId={category.id}
            bind:editingItem
            bind:editingContent
            onVote={onVoteItem}
            onDelete={onDeleteItem}
            {onStartEdit}
            {onSaveEdit}
            {onCancelEdit}
          />
        </div>
      {/each}

      <!-- Add Item Form -->
      <div class="mt-4">
        <AddItemForm laneId={category.id} {onAddItem} />
      </div>
    </CardContent>
  </div>
</Card>

<style>
</style>

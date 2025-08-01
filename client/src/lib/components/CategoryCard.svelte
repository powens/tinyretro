<script lang="ts">
  import { Card, CardContent, CardHeader } from "$lib/components/ui/card";
  import { Badge } from "$lib/components/ui/badge";
  import RetroItemCard from "./RetroItemCard.svelte";
  import AddItemForm from "./AddItemForm.svelte";
  import { dndzone, TRIGGERS } from "svelte-dnd-action";
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

  // Create a reactive copy of items for dnd-action
  let items = $state([
    ...category.items.map((item) => ({ ...item, fromLaneId: category.id })),
  ]);

  // Sync items when category changes
  $effect(() => {
    items = [
      ...category.items.map((item) => ({ ...item, fromLaneId: category.id })),
    ];
  });

  function handleDndConsider(e: CustomEvent) {
    items = e.detail.items;
  }

  function handleDndFinalize(e: CustomEvent) {
    items = e.detail.items;

    // Check if this was a move between categories or reorder within same category
    const draggedItem = e.detail.info.draggedElement;
    const wasDroppedFromOutside =
      e.detail.info.trigger === TRIGGERS.DROPPED_INTO_ZONE;

    if (
      wasDroppedFromOutside &&
      draggedItem.fromLaneId &&
      draggedItem.fromLaneId !== category.id
    ) {
      // Item moved from different category
      const newIndex = items.findIndex((item) => item.id === draggedItem.id);
      onMoveItem(draggedItem.id, draggedItem.fromLaneId, category.id, newIndex);
    } else if (e.detail.info.trigger === TRIGGERS.DROPPED_INTO_ANOTHER) {
      // Item was dragged out to another category - let that category handle it
      return;
    } else {
      // Reorder within same category
      onReorderItems(category.id, items);
    }
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
  <CardContent class="p-4 max-h-96 overflow-y-auto">
    <div
      use:dndzone={{
        items: items,
        flipDurationMs: 300,
        dropTargetStyle: {},
        dragDisabled: false,
        morphDisabled: false,
        centreDraggedOnCursor: true,
        dropFromOthersDisabled: false,
        transformDraggedElement: (draggedEl) => {
          // Add category info to dragged element for cross-category moves
          if (draggedEl) {
            draggedEl.dataset.fromLaneId = category.id;
          }
        },
      }}
      onconsider={handleDndConsider}
      onfinalize={handleDndFinalize}
      class="space-y-3 min-h-[100px] dnd-container"
    >
      {#each items as item (item.id)}
        <div animate:flip={{ duration: 300 }}>
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
    </div>

    <!-- Add Item Form -->
    <div class="mt-4">
      <AddItemForm laneId={category.id} {onAddItem} />
    </div>
  </CardContent>
</Card>

<style>
  /* Hide any drop zones that appear outside of cards */
  :global(body > .dnd-drop-zone),
  :global(.dnd-drop-zone-placeholder),
  :global([data-is-dnd-shadow-placeholder]) {
    display: none !important;
  }

  /* Style drop zones only within our container */
  .dnd-container :global([data-is-dnd-shadow-placeholder]) {
    display: block !important;
    min-height: 60px;
    background-color: rgba(59, 130, 246, 0.05);
    border: 2px dashed rgba(59, 130, 246, 0.3);
    border-radius: 8px;
    margin: 8px 0;
    display: flex;
    align-items: center;
    justify-content: center;
    color: rgba(59, 130, 246, 0.7);
    font-size: 14px;
    font-weight: 500;
    transition: all 0.2s ease;
  }

  .dnd-container :global([data-is-dnd-shadow-placeholder]::before) {
    content: "Drop item here";
  }

  /* Ensure the container has proper overflow handling */
  .dnd-container {
    position: relative;
    overflow: visible;
  }

  /* Style for dragged items */
  :global([data-is-dnd-shadow-item]) {
    opacity: 0.5;
    transform: rotate(2deg);
    transition: all 0.2s ease;
  }
</style>

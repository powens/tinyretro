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

  // Track if we're currently dragging to avoid conflicts with server updates
  let isDragging = $state(false);

  // Sync items when category changes, but only if we're not dragging
  $effect(() => {
    if (!isDragging) {
      items = [
        ...category.items.map((item) => ({ ...item, fromLaneId: category.id })),
      ];
    }
  });

  function handleDndConsider(e: CustomEvent) {
    console.log(`Consider: ${category.title}`, {
      items: e.detail.items.length,
      trigger: e.detail.info?.trigger,
      draggedElement: e.detail.info?.draggedElement?.id,
    });
    isDragging = true;
    // Only update items array, don't trigger server actions during consider phase
    items = e.detail.items;
  }

  function handleDndFinalize(e: CustomEvent) {
    console.log(`Finalize: ${category.title}`, {
      items: e.detail.items.length,
      trigger: e.detail.info?.trigger,
      draggedElement: e.detail.info?.draggedElement?.id,
    });
    const finalItems = e.detail.items;
    items = finalItems;

    // Small delay to ensure DOM updates complete before resetting drag state
    setTimeout(() => {
      isDragging = false;
    }, 100);

    // Check if this was a move between categories or reorder within same category
    const draggedElement = e.detail.info.draggedElement;
    const wasDroppedFromOutside =
      e.detail.info.trigger === TRIGGERS.DROPPED_INTO_ZONE;

    if (wasDroppedFromOutside && draggedElement) {
      // Extract the source lane ID from the DOM element's dataset
      const fromLaneId = draggedElement.dataset?.fromLaneId;
      const draggedItemId = draggedElement.dataset?.id;

      if (fromLaneId && draggedItemId && fromLaneId !== category.id) {
        // Item moved from different category
        const newIndex = finalItems.findIndex(
          (item) => item.id === draggedItemId
        );
        onMoveItem(draggedItemId, fromLaneId, category.id, newIndex);
      } else {
        // Reorder within same category or missing data
        onReorderItems(category.id, finalItems);
      }
    } else if (e.detail.info.trigger === TRIGGERS.DROPPED_INTO_ANOTHER) {
      // Item was dragged out to another category - let that category handle it
      console.log(`Item dragged out of ${category.title}`);
      return;
    } else {
      // Reorder within same category
      onReorderItems(category.id, finalItems);
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
        morphDisabled: true,
        centreDraggedOnCursor: true,
        dropFromOthersDisabled: false,
        transformDraggedElement: (draggedEl, draggedItem) => {
          // Add category info to dragged element for cross-category moves
          if (draggedEl) {
            draggedEl.dataset.fromLaneId = category.id;

            // Set the item ID from the draggedItem data
            if (draggedItem && draggedItem.id) {
              draggedEl.dataset.id = draggedItem.id;
            }
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
</style>

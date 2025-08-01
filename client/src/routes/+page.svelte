<script lang="ts">
  import { onMount } from "svelte";
  import RetroHeader from "$lib/components/RetroHeader.svelte";
  import AddCategoryModal from "$lib/components/AddCategoryModal.svelte";
  import CategoryCard from "$lib/components/CategoryCard.svelte";
  import EmptyState from "$lib/components/EmptyState.svelte";
  import WebSocketConnection from "$lib/components/WebSocketConnection.svelte";
  import type { Board, AllActions } from "$lib/BoardState.svelte";

  // WebSocket connection - managed by WebSocketConnection component
  let socketState = $state(
    "disconnected" as "connected" | "disconnected" | "connecting"
  );
  let sendAction = $state<(action: AllActions) => void>(() => {
    console.error("sendAction not initialized");
  });
  let boardState = $state({}) as Board;

  // UI State
  let isDarkMode = $state(false);
  let showAddCategory = $state(false);
  let newCategoryTitle = $state("");
  let editingItem = $state<string | null>(null);
  let editingContent = $state("");

  // Theme colors for different lane types
  const themeColors = {
    "went-well": "bg-emerald-500",
    "to-improve": "bg-amber-500",
    "action-items": "bg-blue-500",
  };

  // Functions
  function addCategory() {
    if (newCategoryTitle.trim() && sendAction) {
      // ActionAddLane type is missing the type field in the backend definition
      const action = {
        title: newCategoryTitle.trim(),
      };
      sendAction(action as AllActions);
      newCategoryTitle = "";
      showAddCategory = false;
    }
  }

  function handleAddCategoryCancel() {
    showAddCategory = false;
    newCategoryTitle = "";
  }

  function addItem(laneId: string, content: string) {
    if (content.trim() && sendAction) {
      sendAction({
        type: "AddItem",
        lane_id: laneId,
        body: content.trim(),
      });
    }
  }

  function voteItem(laneId: string, itemId: string) {
    if (sendAction) {
      sendAction({
        type: "UpvoteItem",
        lane_id: laneId,
        id: itemId,
      });
    }
  }

  function deleteItem(laneId: string, itemId: string) {
    if (sendAction) {
      // ActionRemoveItem type is missing the type field in the backend definition
      const action = {
        lane_id: laneId,
        id: itemId,
      };
      sendAction(action as AllActions);
    }
  }

  function startEdit(item: {
    id: string;
    content: string;
    votes: number;
    sortOrder: number;
  }) {
    editingItem = item.id;
    editingContent = item.content;
  }

  function saveEdit(laneId: string, itemId: string) {
    // Note: The current backend doesn't have an edit action, so we'll need to
    // remove the old item and add a new one as a workaround
    if (editingContent.trim() && socketState === "connected") {
      deleteItem(laneId, itemId);
      addItem(laneId, editingContent.trim());
    }
    editingItem = null;
    editingContent = "";
  }

  function cancelEdit() {
    editingItem = null;
    editingContent = "";
  }

  // NEW: Simplified drag and drop handlers for svelte-dnd-action
  function handleMoveItem(itemId: string, fromLaneId: string, toLaneId: string, _newIndex?: number) {
    if (!sendAction) return;

    // Add the fromLaneId to the item for cross-category moves
    sendAction({
      type: "MoveItem",
      from_lane_id: fromLaneId,
      to_lane_id: toLaneId,
      item_id: itemId,
    });
  }

  function handleReorderItems(laneId: string, items: Array<{id: string; sortOrder: number}>) {
    if (!sendAction) return;

    // Send reorder actions for each item that changed position
    items.forEach((item, index) => {
      if (item.sortOrder !== index) {
        sendAction({
          type: "ReorderItem",
          lane_id: laneId,
          item_id: item.id,
          new_position: index,
        });
      }
    });
  }

  // Convert board lanes to display format
  let lanes = $derived(
    boardState?.lanes
      ? Object.entries(boardState.lanes).map(([id, lane]) => ({
          id,
          title: lane.title,
          theme: lane.theme,
          color:
            themeColors[lane.theme as keyof typeof themeColors] ||
            "bg-purple-500",
          items: Object.entries(lane.items)
            .map(([itemId, item]) => ({
              id: itemId,
              content: item.body,
              votes: item.vote_count,
              sortOrder: item.sort_order,
            }))
            .sort((a, b) => a.sortOrder - b.sortOrder),
        }))
      : []
  );

  onMount(() => {
    // Check for saved theme preference
    const savedTheme = localStorage.getItem("theme");
    if (
      savedTheme === "dark" ||
      (!savedTheme && window.matchMedia("(prefers-color-scheme: dark)").matches)
    ) {
      isDarkMode = true;
      document.documentElement.classList.add("dark");
    }
  });
</script>

<svelte:head>
  <title>Sprint Retrospective Board</title>
</svelte:head>

<div
  class="min-h-screen bg-gray-50 dark:bg-gray-900 transition-colors duration-200"
>
  <!-- WebSocket Connection Management -->
  <WebSocketConnection bind:boardState bind:socketState bind:sendAction />

  <!-- Header -->
  <RetroHeader bind:isDarkMode onAddCategory={() => (showAddCategory = true)} />

  <!-- Main Content -->
  <main class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
    <!-- Add Category Modal -->
    <AddCategoryModal
      bind:open={showAddCategory}
      bind:categoryTitle={newCategoryTitle}
      onAddCategory={addCategory}
      onCancel={handleAddCategoryCancel}
    />

    <!-- Categories Grid -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
      {#each lanes as category (category.id)}
        <CategoryCard
          {category}
          bind:editingItem
          bind:editingContent
          onMoveItem={handleMoveItem}
          onReorderItems={handleReorderItems}
          onAddItem={addItem}
          onVoteItem={voteItem}
          onDeleteItem={deleteItem}
          onStartEdit={startEdit}
          onSaveEdit={saveEdit}
          onCancelEdit={cancelEdit}
        />
      {/each}
    </div>

    <!-- Empty State -->
    {#if lanes.length === 0}
      <EmptyState onAddCategory={() => (showAddCategory = true)} />
    {/if}
  </main>
</div>

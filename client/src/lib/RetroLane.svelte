<script lang="ts">
  import type {
    Lane,
    ActionMoveItem,
    ActionReorderItem,
    AllActions,
  } from "./BoardState.svelte";
  import RetroItem from "./RetroItem.svelte";
  import AddItem from "./AddItem.svelte";
  import { getContext } from "svelte";
  import { flip } from "svelte/animate";
  import { quintOut } from "svelte/easing";

  let { laneId, lane }: { laneId: string; lane: Lane } = $props();

  const sendAction =
    getContext<() => (action: AllActions) => void>("sendAction");

  let isDragOver = $state(false);
  let dragOverIndex = $state<number | null>(null);

  // Sort items by sort_order for display
  let sortedItems = $derived(
    Object.entries(lane.items).sort(
      ([, a], [, b]) => a.sort_order - b.sort_order
    )
  );

  function handleDragOver(event: DragEvent) {
    event.preventDefault(); // Allow drop
    console.log(`Drag over lane ${laneId}`);
    isDragOver = true;
  }

  function handleDragLeave(event: DragEvent) {
    // Only set isDragOver to false if we're actually leaving the drop zone
    // and not just moving between child elements
    if (!event.currentTarget?.contains(event.relatedTarget as Node)) {
      console.log(`Drag leave lane ${laneId}`);
      isDragOver = false;
      dragOverIndex = null;
    }
  }

  function handleDrop(event: DragEvent) {
    event.preventDefault();
    console.log(`Drop in lane ${laneId}`);
    isDragOver = false;
    dragOverIndex = null;

    const itemId = event.dataTransfer?.getData("text/item-id");
    const fromLaneId = event.dataTransfer?.getData("text/lane-id");

    console.log(
      `Lane drop - Item: ${itemId}, From: ${fromLaneId}, To: ${laneId}`
    );

    if (itemId && fromLaneId) {
      if (fromLaneId !== laneId) {
        // Moving between lanes
        console.log(`Moving item ${itemId} from ${fromLaneId} to ${laneId}`);
        const action: ActionMoveItem = {
          type: "MoveItem",
          from_lane_id: fromLaneId,
          to_lane_id: laneId,
          item_id: itemId,
        };
        sendAction()(action);
      }
      // Note: Reordering within the same lane is handled by drop zones between items
    }
  }

  function handleDropZoneDragOver(event: DragEvent, index: number) {
    event.preventDefault();
    event.stopPropagation();
    console.log(`Drag over drop zone at index ${index} in lane ${laneId}`);
    dragOverIndex = index;
  }

  function handleDropZoneDragLeave(event: DragEvent) {
    // Only clear if we're actually leaving the drop zone
    if (!event.currentTarget?.contains(event.relatedTarget as Node)) {
      console.log(`Drag leave drop zone in lane ${laneId}`);
      dragOverIndex = null;
    }
  }

  function handleDropZoneDrop(event: DragEvent, newPosition: number) {
    event.preventDefault();
    event.stopPropagation();
    console.log(`Drop at position ${newPosition} in lane ${laneId}`);
    dragOverIndex = null;

    const itemId = event.dataTransfer?.getData("text/item-id");
    const fromLaneId = event.dataTransfer?.getData("text/lane-id");

    console.log(
      `Item: ${itemId}, From Lane: ${fromLaneId}, To Lane: ${laneId}`
    );

    if (itemId && fromLaneId === laneId) {
      // Reordering within the same lane
      console.log(
        `Reordering item ${itemId} in lane ${laneId} to position ${newPosition}`
      );
      const action: ActionReorderItem = {
        type: "ReorderItem",
        lane_id: laneId,
        item_id: itemId,
        new_position: newPosition,
      };
      console.log("Sending ReorderItem action:", action);
      sendAction()(action);
    } else if (itemId && fromLaneId && fromLaneId !== laneId) {
      // Moving between lanes - also handle this in drop zones
      console.log(
        `Moving item ${itemId} from ${fromLaneId} to ${laneId} at position ${newPosition}`
      );
      const action: ActionMoveItem = {
        type: "MoveItem",
        from_lane_id: fromLaneId,
        to_lane_id: laneId,
        item_id: itemId,
      };
      sendAction()(action);
    }
  }
</script>

<div class="lane">
  <h3 class="title">{lane.title}</h3>
  <div
    class="items-container drop-zone {isDragOver ? 'drag-over' : ''}"
    data-lane-id={laneId}
    role="region"
    aria-label="Drop zone for {lane.title}"
    ondragover={handleDragOver}
    ondragleave={handleDragLeave}
    ondrop={handleDrop}
  >
    <!-- Drop zone at the beginning of the lane -->
    <div
      class="item-drop-zone {dragOverIndex === 0 ? 'drag-over' : ''}"
      ondragover={(e) => handleDropZoneDragOver(e, 0)}
      ondragleave={handleDropZoneDragLeave}
      ondrop={(e) => handleDropZoneDrop(e, 0)}
      role="button"
      tabindex="-1"
      aria-label="Drop zone before first item"
    ></div>

    {#each sortedItems as [itemId, item], index (itemId)}
      <div animate:flip={{ duration: 300, easing: quintOut }}>
        <RetroItem
          body={item.body}
          vote_count={item.vote_count}
          theme={lane.theme}
          {laneId}
          id={itemId}
        />

        <!-- Drop zone after this item -->
        <div
          class="item-drop-zone {dragOverIndex === index + 1
            ? 'drag-over'
            : ''}"
          ondragover={(e) => handleDropZoneDragOver(e, index + 1)}
          ondragleave={handleDropZoneDragLeave}
          ondrop={(e) => handleDropZoneDrop(e, index + 1)}
          role="button"
          tabindex="-1"
          aria-label="Drop zone after item {index + 1}"
        ></div>
      </div>
    {/each}

    <AddItem {laneId} />
  </div>
</div>

<style>
  .lane {
    min-height: 200px;
  }

  .title {
    font-size: 2rem;
  }

  .items-container {
    min-height: 150px;
    padding: 0.5rem;
    border-radius: 0.25rem;
    transition: background-color 0.2s ease;
  }

  .drop-zone {
    border: 2px dashed transparent;
    transition: all 0.2s ease;
  }

  .drop-zone.drag-over {
    border-color: #007acc;
    background-color: rgba(0, 122, 204, 0.1);
    transform: scale(1.02);
  }

  .item-drop-zone {
    height: 8px;
    margin: 4px 0;
    border-radius: 2px;
    transition: all 0.2s ease;
    background-color: rgba(0, 122, 204, 0.1);
    border: 1px dashed rgba(0, 122, 204, 0.3);
    opacity: 0.3;
  }

  .item-drop-zone.drag-over {
    opacity: 1;
    height: 12px;
    background-color: #007acc;
    border-color: #007acc;
    box-shadow: 0 0 8px rgba(0, 122, 204, 0.5);
    transform: scaleY(1.5);
  }
</style>

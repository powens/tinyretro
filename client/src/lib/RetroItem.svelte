<script lang="ts">
  import { ThumbsUp, Check } from "lucide-svelte";
  import { Button } from "kampsy-ui";
  import { getContext } from "svelte";
  import type { ActionUpvoteItem, SendActionFunc } from "./BoardState.svelte";

  export let body: string;
  export let vote_count: number;
  export let theme: string;
  export let laneId: string;
  export let id: string;

  let hasVoted = false;
  let isDragging = false;
  const sendAction = getContext<SendActionFunc>("sendAction");

  // HTML5 Drag and Drop handlers
  function handleDragStart(event: DragEvent) {
    if (!event.dataTransfer) return;

    console.log("Drag started for item:", id, "in lane:", laneId);
    isDragging = true;

    // Set the data to be transferred
    event.dataTransfer.setData("text/item-id", id);
    event.dataTransfer.setData("text/lane-id", laneId);

    console.log("Set drag data - item-id:", id, "lane-id:", laneId);

    // Set drag effect
    event.dataTransfer.effectAllowed = "move";
  }

  function handleDragEnd(event: DragEvent) {
    console.log("Drag ended for item:", id);
    isDragging = false;
  }
</script>

<div
  class="item {theme} {isDragging ? 'dragging' : ''}"
  draggable="true"
  ondragstart={handleDragStart}
  ondragend={handleDragEnd}
  data-item-id={id}
  data-lane-id={laneId}
  role="button"
  tabindex="0"
  aria-label="Drag to move item: {body}"
>
  <div class="drag-handle" title="Drag to move">⋮⋮</div>
  <p class="body">{body}</p>
  <div class="footer">
    <Button
      class="upvote"
      type="secondary"
      on:click={() => {
        const action: ActionUpvoteItem = {
          type: "UpvoteItem",
          lane_id: laneId,
          id: id,
        };
        sendAction(action);

        hasVoted = true;
        setTimeout(() => {
          hasVoted = false;
        }, 5000);
      }}
    >
      {#if hasVoted}
        <Check class="icon" />
      {:else}
        <ThumbsUp class="icon" />
      {/if}
      {vote_count}
    </Button>
  </div>
</div>

<style>
  .item {
    padding: 0.5rem;
    border: 1px solid;
    margin-bottom: 1rem;
    position: relative;
    cursor: move;
    transition: all 0.2s ease;
  }

  .item.dragging {
    opacity: 0.8;
    transform: rotate(5deg);
    z-index: 1000;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
  }

  .drag-handle {
    position: absolute;
    top: 0.25rem;
    right: 0.25rem;
    cursor: grab;
    font-size: 0.875rem;
    color: #666;
    line-height: 1;
    user-select: none;
    padding: 0.25rem;
    border-radius: 0.25rem;
    transition: all 0.2s ease;
  }

  .drag-handle:hover {
    background-color: rgba(0, 0, 0, 0.1);
    color: #333;
  }

  .drag-handle:active {
    cursor: grabbing;
    background-color: rgba(0, 0, 0, 0.2);
  }

  .body {
    margin-bottom: 0.5rem;
    padding-right: 1rem; /* Make space for the drag handle */
  }
  .footer {
    display: flex;
    align-items: flex-end;
    justify-content: end;
  }
  .icon {
    padding-right: 1rem;
  }
  .upvote {
    display: flex;
  }
  .upvote:hover {
    color: red;
  }

  .went-well {
    background-color: #d4edda;
    border-color: #c3e6cb;
    color: #000;
  }

  .to-improve {
    background-color: #f8d7da;
    border-color: #f5c6cb;
    color: #000;
  }

  .action-items {
    background-color: #cce5ff;
    border-color: #b8daff;
    color: #000;
  }
</style>

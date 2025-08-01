<script lang="ts">
  import { scale } from "svelte/transition";
  import { ThumbsUp, Edit2, Trash2, GripVertical } from "lucide-svelte";
  import { Button } from "$lib/components/ui/button";
  import { Textarea } from "$lib/components/ui/textarea";

  type Item = {
    id: string;
    content: string;
    votes: number;
    sortOrder: number;
  };

  let {
    item,
    laneId,
    editingItem = $bindable(),
    editingContent = $bindable(),
    onVote,
    onDelete,
    onStartEdit,
    onSaveEdit,
    onCancelEdit,
  }: {
    item: Item;
    laneId: string;
    editingItem: string | null;
    editingContent: string;
    onVote: (laneId: string, itemId: string) => void;
    onDelete: (laneId: string, itemId: string) => void;
    onStartEdit: (item: Item) => void;
    onSaveEdit: (laneId: string, itemId: string) => void;
    onCancelEdit: () => void;
  } = $props();

  let isEditing = $derived(editingItem === item.id);
  let showVoted = $state(false);

  function handleVote() {
    onVote(laneId, item.id);
    showVoted = true;
    setTimeout(() => {
      showVoted = false;
    }, 1000);
  }

  function handleEdit() {
    onStartEdit(item);
  }

  function handleSave() {
    onSaveEdit(laneId, item.id);
  }

  function handleCancel() {
    onCancelEdit();
  }

  function handleDelete() {
    onDelete(laneId, item.id);
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Enter" && !e.shiftKey) {
      e.preventDefault();
      handleSave();
    }
    if (e.key === "Escape") {
      handleCancel();
    }
  }
</script>

<div
  class="retro-item bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg p-4 shadow-sm hover:shadow-md transition-all duration-200"
  transition:scale={{ duration: 200 }}
  role="button"
  tabindex="0"
  aria-label="Retro item: {item.content}"
>
  <div class="drag-handle" aria-label="Drag to reorder">
    <GripVertical size={16} />
  </div>

  <div class="content">
    {#if isEditing}
      <div class="edit-form">
        <Textarea
          bind:value={editingContent}
          placeholder="Edit your item..."
          class="mb-2 min-h-[60px] resize-none"
          autofocus
          onkeydown={handleKeydown}
        />
        <div class="flex gap-2">
          <Button size="sm" onclick={handleSave}>Save</Button>
          <Button size="sm" variant="outline" onclick={handleCancel}>
            Cancel
          </Button>
        </div>
      </div>
    {:else}
      <p class="item-text text-gray-900 dark:text-gray-100">{item.content}</p>
    {/if}
  </div>

  <div class="actions">
    <Button
      size="sm"
      variant="ghost"
      class="vote-button {showVoted ? 'voted' : ''}"
      onclick={handleVote}
      aria-label="Vote for this item"
    >
      <ThumbsUp size={16} class={showVoted ? "text-blue-500" : ""} />
      <span class="vote-count">{item.votes}</span>
    </Button>

    {#if !isEditing}
      <Button
        size="sm"
        variant="ghost"
        onclick={handleEdit}
        aria-label="Edit this item"
      >
        <Edit2 size={16} />
      </Button>
    {/if}

    <Button
      size="sm"
      variant="ghost"
      onclick={handleDelete}
      aria-label="Delete this item"
      class="delete-button hover:text-red-500"
    >
      <Trash2 size={16} />
    </Button>
  </div>
</div>

<style>
  .retro-item {
    display: flex;
    align-items: flex-start;
    gap: 12px;
    cursor: grab;
  }

  .retro-item:active {
    cursor: grabbing;
  }

  .drag-handle {
    display: flex;
    align-items: center;
    color: #9ca3af;
    cursor: grab;
    padding: 4px;
    border-radius: 4px;
    transition: all 0.2s ease;
  }

  .drag-handle:hover {
    color: #6b7280;
    background-color: #f3f4f6;
  }

  .content {
    flex: 1;
    min-width: 0;
  }

  .item-text {
    margin: 0;
    line-height: 1.5;
    word-wrap: break-word;
  }

  .edit-form {
    width: 100%;
  }

  .actions {
    display: flex;
    align-items: center;
    gap: 4px;
    flex-shrink: 0;
  }

  :global(.vote-button) {
    display: flex;
    align-items: center;
    gap: 6px;
    transition: all 0.2s ease;
  }

  :global(.vote-button.voted) {
    background-color: rgba(59, 130, 246, 0.1);
    color: rgb(59, 130, 246);
  }

  .vote-count {
    font-size: 12px;
    min-width: 16px;
    text-align: center;
  }

  /* Drag feedback from svelte-dnd-action */
  :global([data-is-dnd-shadow-item]) .retro-item {
    opacity: 0.5;
    transform: rotate(5deg);
  }
</style>

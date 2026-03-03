<script lang="ts">
  import type {
    ActionUpvoteItem,
    ActionEditItem,
    ActionRemoveItem,
    ActionMergeItems,
    SendActionFunc,
  } from "$lib/BoardState.svelte";
  import { getMergeContext } from "./merge-context";
  import { onDestroy } from "svelte";
  import {
    ThumbsUp,
    Check,
    GripVertical,
    Pencil,
    X,
    Trash2,
    Merge,
  } from "@lucide/svelte";
  import Button from "./Button.svelte";

  const {
    id,
    body,
    vote_count,
    laneId,
    sendAction,
    cardBg = "",
    cardBorder = "",
  }: {
    id: string;
    body: string;
    vote_count: number;
    laneId: string;
    sendAction: SendActionFunc;
    cardBg?: string;
    cardBorder?: string;
  } = $props();

  const merge = getMergeContext();

  // Derived merge state from context
  let isMergeSource = $derived(
    merge.source?.laneId === laneId && merge.source?.itemId === id,
  );
  let isMergeTarget = $derived(
    merge.source !== null &&
      merge.source.laneId === laneId &&
      merge.source.itemId !== id,
  );

  // -- Item mode (discriminated union enforces mutual exclusivity) --
  type ItemMode =
    | { type: "view" }
    | { type: "editing"; text: string }
    | { type: "confirming-delete" }
    | { type: "merge-editing"; text: string };

  let mode: ItemMode = $state({ type: "view" });
  let hasVoted = $state(false);
  let voteTimer: ReturnType<typeof setTimeout> | undefined;
  let textareaEl: HTMLTextAreaElement | undefined = $state();
  let mergeTextareaEl: HTMLTextAreaElement | undefined = $state();

  // -- Editing --
  function startEditing() {
    mode = { type: "editing", text: body };
    requestAnimationFrame(() => {
      textareaEl?.focus();
      textareaEl?.select();
    });
  }

  function saveEdit() {
    if (mode.type !== "editing") return;
    const trimmed = mode.text.trim();
    if (!trimmed || trimmed === body) {
      resetMode();
      return;
    }
    const action: ActionEditItem = {
      type: "EditItem",
      lane_id: laneId,
      id,
      body: trimmed,
    };
    sendAction(action);
    resetMode();
  }

  function handleEditKeydown(e: KeyboardEvent) {
    if (e.key === "Enter" && (e.metaKey || e.ctrlKey)) {
      e.preventDefault();
      saveEdit();
    }
    if (e.key === "Escape") {
      resetMode();
    }
  }

  // -- Delete --
  function requestDelete() {
    mode = { type: "confirming-delete" };
  }

  function confirmDelete() {
    const action: ActionRemoveItem = {
      type: "RemoveItem",
      lane_id: laneId,
      id,
    };
    sendAction(action);
    resetMode();
  }

  // -- Upvote --
  function handleUpvote() {
    const action: ActionUpvoteItem = {
      type: "UpvoteItem",
      lane_id: laneId,
      id,
    };
    sendAction(action);
    hasVoted = true;
    clearTimeout(voteTimer);
    voteTimer = setTimeout(() => {
      hasVoted = false;
    }, 3000);
  }

  // -- Merge --
  function handleStartMerge() {
    merge.start(laneId, id, body, vote_count);
  }

  function handleSelectAsTarget() {
    if (!merge.source) return;
    const combined = `${merge.source.body}\n\n${body}`;
    mode = { type: "merge-editing", text: combined };
    requestAnimationFrame(() => {
      mergeTextareaEl?.focus();
    });
  }

  function confirmMerge() {
    if (mode.type !== "merge-editing" || !merge.source) return;
    const trimmed = mode.text.trim();
    if (!trimmed) return;
    const action: ActionMergeItems = {
      type: "MergeItems",
      lane_id: laneId,
      source_id: merge.source.itemId,
      target_id: id,
      merged_body: trimmed,
    };
    sendAction(action);
    resetMode();
    merge.cancel();
  }

  function cancelMerge() {
    resetMode();
    merge.cancel();
  }

  function handleMergeKeydown(e: KeyboardEvent) {
    if (e.key === "Enter" && (e.metaKey || e.ctrlKey)) {
      e.preventDefault();
      confirmMerge();
    }
    if (e.key === "Escape") {
      cancelMerge();
    }
  }

  // -- Shared --
  function resetMode() {
    mode = { type: "view" };
  }

  onDestroy(() => {
    clearTimeout(voteTimer);
  });
</script>

<div
  class="card p-3 group transition-all duration-200 hover:shadow-md
    {isMergeSource
    ? 'cursor-default ring-2 ring-primary-500 shadow-lg shadow-primary-500/20'
    : isMergeTarget
      ? 'cursor-pointer'
      : 'cursor-grab active:cursor-grabbing'}
    border {cardBorder || 'border-surface-300 dark:border-surface-600'}
    {cardBg || 'bg-surface-100 dark:bg-surface-800'}
    rounded-lg"
  aria-label="Retro item: {body}"
>
  <!-- Merge editor overlay (target item, editing combined text) -->
  {#if mode.type === "merge-editing"}
    <div class="flex flex-col gap-3 py-2">
      <div
        class="flex items-center gap-2 text-primary-600 dark:text-primary-400"
      >
        <Merge size={18} strokeWidth={2} />
        <p class="text-sm font-bold">Merge items</p>
      </div>
      <p class="text-xs opacity-60">
        Edit the combined text below. Votes will be added together ({merge
          .source?.vote_count ?? 0} + {vote_count}).
      </p>
      <textarea
        bind:this={mergeTextareaEl}
        class="textarea w-full text-sm"
        rows="4"
        bind:value={mode.text}
        onkeydown={handleMergeKeydown}
      ></textarea>
      <div class="flex gap-2 justify-end">
        <Button variant="filled-surface" onclick={cancelMerge}>
          <X size={12} />
          Cancel
        </Button>
        <Button variant="filled-primary" onclick={confirmMerge}>
          <Merge size={12} />
          Merge
        </Button>
      </div>
      <p class="text-xs opacity-40 text-right">
        Ctrl+Enter to merge · Esc to cancel
      </p>
    </div>

    <!-- Merge source highlight -->
  {:else if isMergeSource}
    <div class="flex flex-col items-center justify-center gap-3 py-4 px-3">
      <div
        class="flex items-center gap-2 text-primary-600 dark:text-primary-400"
      >
        <Merge size={18} strokeWidth={2} />
        <p class="text-sm font-bold">Merge source selected</p>
      </div>
      <p class="text-xs opacity-60 text-center">
        Click another item in this lane to merge into it.
      </p>
      <Button variant="filled-surface" onclick={() => merge.cancel()}>
        <X size={12} />
        Cancel merge
      </Button>
    </div>

    <!-- Merge target overlay (clickable to open merge editor) -->
  {:else if isMergeTarget}
    <button class="w-full text-left" onclick={handleSelectAsTarget}>
      <div class="flex gap-2">
        <div class="flex-shrink-0 opacity-40 pt-0.5">
          <GripVertical size={16} />
        </div>
        <div class="flex-1 min-w-0">
          <p class="text-sm leading-relaxed">{body}</p>
          <div class="flex items-center justify-between mt-2 gap-2">
            <div class="flex items-center gap-1.5 text-primary-500">
              <Merge size={14} />
              <span class="text-xs font-semibold">Merge into this</span>
            </div>
            <div class="flex items-center gap-1 text-xs opacity-60">
              <ThumbsUp size={12} />
              <span class="font-mono">{vote_count}</span>
            </div>
          </div>
        </div>
      </div>
    </button>

    <!-- Delete confirmation overlay -->
  {:else if mode.type === "confirming-delete"}
    <div
      class="flex flex-col items-center justify-center gap-3 py-4 px-3 border-2 border-error-500/50 bg-error-500/10 rounded-lg"
    >
      <div class="flex items-center gap-2 text-error-600 dark:text-error-400">
        <Trash2 size={18} strokeWidth={2} />
        <p class="text-sm font-bold">Delete this item?</p>
      </div>
      <p class="text-xs opacity-60 text-center">
        This action cannot be undone.
      </p>
      <div class="flex gap-2">
        <Button variant="filled-surface" onclick={resetMode}>Cancel</Button>
        <Button variant="filled-error" onclick={confirmDelete}>
          <Trash2 size={13} />
          Delete
        </Button>
      </div>
    </div>

    <!-- Normal content -->
  {:else}
    <div class="flex gap-2">
      <!-- Drag handle indicator -->
      <div
        class="flex-shrink-0 opacity-0 group-hover:opacity-40 transition-opacity pt-0.5"
      >
        <GripVertical size={16} />
      </div>

      <!-- Content -->
      <div class="flex-1 min-w-0">
        {#if mode.type === "editing"}
          <textarea
            bind:this={textareaEl}
            class="textarea w-full text-sm"
            rows="2"
            bind:value={mode.text}
            onkeydown={handleEditKeydown}
            onblur={saveEdit}
          ></textarea>
          <div class="flex gap-1.5 justify-end mt-1.5">
            <Button
              variant="filled-surface"
              onmousedown={(e) => {
                e.preventDefault();
                resetMode();
              }}
            >
              <X size={12} />
              Cancel
            </Button>
            <Button
              variant="filled-primary"
              onmousedown={(e) => {
                e.preventDefault();
                saveEdit();
              }}
            >
              <Check size={12} />
              Save
            </Button>
          </div>
          <p class="text-xs opacity-40 text-right mt-0.5">
            Ctrl+Enter to save · Esc to cancel
          </p>
        {:else}
          <p
            class="text-sm leading-relaxed cursor-text rounded px-1 -mx-1 hover:bg-surface-200/50 dark:hover:bg-surface-700/50 transition-colors"
            ondblclick={startEditing}
            title="Double-click to edit"
          >
            {body}
          </p>
        {/if}

        <!-- Footer -->
        <div class="flex items-center justify-between mt-2 gap-2">
          <div class="flex gap-1">
            {#if mode.type === "view"}
              <!-- ghost variant requires a parent with the Tailwind `group` class to toggle visibility -->
              <Button variant="ghost" onclick={startEditing} title="Edit item">
                <Pencil size={13} />
              </Button>
              <Button
                variant="ghost"
                onclick={handleStartMerge}
                title="Merge with another item"
                class="hover:!text-primary-500"
              >
                <Merge size={13} />
              </Button>
              <Button
                variant="ghost"
                onclick={requestDelete}
                title="Delete item"
                class="hover:!text-error-500"
              >
                <Trash2 size={13} />
              </Button>
            {:else}
              <div></div>
            {/if}
          </div>
          <Button
            variant={hasVoted ? "filled-success" : "filled-surface"}
            onclick={handleUpvote}
          >
            <span class={hasVoted ? "" : "hidden"}
              ><Check size={14} strokeWidth={2.5} /></span
            >
            <span class={hasVoted ? "hidden" : ""}
              ><ThumbsUp size={14} strokeWidth={2} /></span
            >
            <span class="text-xs font-bold font-mono">{vote_count}</span>
          </Button>
        </div>
      </div>
    </div>
  {/if}
</div>

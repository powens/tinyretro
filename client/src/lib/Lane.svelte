<script lang="ts">
  import type {
    Lane,
    DndItem,
    ActionAddItem,
    SendActionFunc,
  } from "$lib/BoardState.svelte";
  import { getLaneTheme } from "./v2-theme";
  import { getMergeContext } from "./merge-context";
  import Item from "./Item.svelte";
  import Button from "./Button.svelte";
  import { dndzone, SHADOW_PLACEHOLDER_ITEM_ID } from "svelte-dnd-action";
  import { flip } from "svelte/animate";

  const {
    laneId,
    lane,
    sendAction,
    items,
    onDndConsider,
    onDndFinalize,
  }: {
    laneId: string;
    lane: Lane;
    sendAction: SendActionFunc;
    items: DndItem[];
    onDndConsider: (items: DndItem[]) => void;
    onDndFinalize: (items: DndItem[]) => void;
  } = $props();

  const merge = getMergeContext();
  let theme = $derived(getLaneTheme(lane.theme));

  let isAdding = $state(false);
  let newBody = $state("");

  const flipDurationMs = 200;

  function submitItem() {
    if (!newBody.trim()) return;
    const action: ActionAddItem = {
      type: "AddItem",
      lane_id: laneId,
      body: newBody,
    };
    sendAction(action);
    newBody = "";
    isAdding = false;
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Enter" && (e.metaKey || e.ctrlKey)) {
      submitItem();
    }
    if (e.key === "Escape") {
      isAdding = false;
      newBody = "";
    }
  }

  function handleConsider(e: CustomEvent<{ items: DndItem[] }>) {
    onDndConsider(e.detail.items);
  }

  function handleFinalize(e: CustomEvent<{ items: DndItem[] }>) {
    onDndFinalize(e.detail.items);
  }
</script>

<div
  class="card overflow-hidden flex flex-col border {theme.laneBorder} {theme.laneBg} rounded-xl"
>
  <!-- Colored accent bar -->
  <div class="h-1.5 w-full {theme.accent}"></div>

  <!-- Lane header -->
  <div
    class="px-4 pt-3 pb-3 border-b border-surface-200/60 dark:border-surface-700/60 {theme.headerBg}"
  >
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-2.5">
        <span class="text-lg">{theme.icon}</span>
        <h3 class="h4 font-bold {theme.headerText}">{lane.title}</h3>
      </div>
      <span class="badge {theme.badge} text-xs font-mono">
        {items.length}
      </span>
    </div>
  </div>

  <!-- Items (dnd zone) -->
  <div
    class="p-3 flex-1 min-h-[200px]"
    use:dndzone={{
      items,
      flipDurationMs,
      dropTargetStyle: {
        outline: "2px dashed rgba(var(--color-primary-500), 0.5)",
        borderRadius: "0.5rem",
      },
    }}
    onconsider={handleConsider}
    onfinalize={handleFinalize}
  >
    {#each items as item (item.id)}
      <div animate:flip={{ duration: flipDurationMs }} class="mb-3">
        {#if item.id === SHADOW_PLACEHOLDER_ITEM_ID}
          <div
            class="card preset-outlined-primary-500 p-3 opacity-50 border-dashed min-h-[60px]"
          ></div>
        {:else}
          <Item
            id={item.id}
            body={item.body}
            vote_count={item.vote_count}
            cardBg={theme.cardBg}
            cardBorder={theme.cardBorder}
            {laneId}
            {sendAction}
          />
        {/if}
      </div>
    {/each}
  </div>

  <!-- Empty state (show only when no items and not adding) -->
  {#if items.length === 0 && !isAdding}
    <div class="text-center py-8 opacity-50">
      <p class="text-sm">No items yet</p>
      <p class="text-xs mt-1">Click + to add one</p>
    </div>
  {/if}

  <!-- Add item section -->
  <div class="p-3 border-t border-surface-200 dark:border-surface-700">
    {#if isAdding}
      <div class="space-y-2">
        <textarea
          class="textarea w-full text-sm"
          placeholder="What's on your mind?"
          rows="3"
          bind:value={newBody}
          onkeydown={handleKeydown}
        ></textarea>
        <div class="flex gap-2 justify-end">
          <Button
            variant="filled-surface"
            onclick={() => {
              isAdding = false;
              newBody = "";
            }}
          >
            Cancel
          </Button>
          <Button
            variant="filled-primary"
            onclick={submitItem}
            disabled={!newBody.trim()}
          >
            Add
          </Button>
        </div>
        <p class="text-xs opacity-50 text-right">Ctrl+Enter to submit</p>
      </div>
    {:else}
      <Button variant="outlined" fullWidth onclick={() => (isAdding = true)}>
        + Add item
      </Button>
    {/if}
  </div>
</div>

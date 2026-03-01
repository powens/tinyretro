<script lang="ts">
  import type {
    Board,
    DndItem,
    MergeSource,
    SendActionFunc,
  } from "$lib/BoardState.svelte";
  import Lane from "./Lane.svelte";
  import { setMergeContext } from "./merge-context";
  import { SHADOW_ITEM_MARKER_PROPERTY_NAME } from "svelte-dnd-action";

  const {
    boardState,
    sendAction,
  }: { boardState: Board; sendAction: SendActionFunc } = $props();

  // -- Merge state (provided via context) --
  let mergeSource: MergeSource = $state(null);

  function startMerge(
    laneId: string,
    itemId: string,
    body: string,
    vote_count: number,
  ) {
    mergeSource = { laneId, itemId, body, vote_count };
  }

  function cancelMerge() {
    mergeSource = null;
  }

  setMergeContext({
    get source() {
      return mergeSource;
    },
    start: startMerge,
    cancel: cancelMerge,
  });

  // -- DnD state --
  let laneItems: Record<string, DndItem[]> = $state({});

  // Sync from boardState -> laneItems whenever boardState changes
  $effect(() => {
    const newLaneItems: Record<string, DndItem[]> = {};
    for (const [laneId, lane] of Object.entries(boardState.lanes)) {
      newLaneItems[laneId] = Object.entries(lane.items)
        .map(([itemId, item]) => ({
          id: itemId,
          body: item.body,
          vote_count: item.vote_count,
          sort_order: item.sort_order,
        }))
        .sort((a, b) => a.sort_order - b.sort_order);
    }
    laneItems = newLaneItems;
  });

  function handleDndConsider(laneId: string, newItems: DndItem[]) {
    laneItems[laneId] = newItems;
  }

  function handleDndFinalize(laneId: string, newItems: DndItem[]) {
    laneItems[laneId] = newItems;

    const lane = boardState.lanes[laneId];
    if (!lane) return;

    for (let i = 0; i < newItems.length; i++) {
      const item = newItems[i];
      if (item[SHADOW_ITEM_MARKER_PROPERTY_NAME as keyof DndItem]) continue;

      const existing = lane.items[item.id];

      if (!existing) {
        // Cross-lane move: item doesn't exist in the destination lane yet.
        // Find the source lane and send a MoveItem action.
        const fromLaneId = Object.entries(boardState.lanes).find(
          ([, l]) => item.id in l.items,
        )?.[0];

        if (fromLaneId) {
          sendAction({
            type: "MoveItem",
            from_lane_id: fromLaneId,
            to_lane_id: laneId,
            item_id: item.id,
          });
          // Server appends to the end of the destination lane.
          // If the item was dropped at an earlier position, reorder it.
          const destLaneSize = Object.keys(lane.items).length;
          if (i !== destLaneSize) {
            sendAction({
              type: "ReorderItem",
              lane_id: laneId,
              item_id: item.id,
              new_position: i,
            });
          }
        }
        break;
      } else if (existing.sort_order !== i) {
        // Same-lane reorder: position changed within this lane.
        sendAction({
          type: "ReorderItem",
          lane_id: laneId,
          item_id: item.id,
          new_position: i,
        });
        break;
      }
    }
  }
</script>

<div class="grid grid-cols-1 md:grid-cols-3 gap-6">
  {#each Object.entries(boardState.lanes) as [laneId, lane] (laneId)}
    <Lane
      {laneId}
      {lane}
      {sendAction}
      items={laneItems[laneId] ?? []}
      onDndConsider={(items) => handleDndConsider(laneId, items)}
      onDndFinalize={(items) => handleDndFinalize(laneId, items)}
    />
  {/each}
</div>

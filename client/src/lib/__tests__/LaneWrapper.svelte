<script lang="ts">
  import Lane from "../Lane.svelte";
  import { setMergeContext, type MergeContext } from "../merge-context";
  import type { Lane as LaneType, DndItem } from "$lib/BoardState.svelte";

  let {
    laneId = "lane-1",
    lane = {
      title: "Went Well",
      theme: "went-well" as const,
      items: {},
    },
    items = [] as DndItem[],
    sendAction = () => {},
    onDndConsider = (_items: DndItem[]) => {},
    onDndFinalize = (_items: DndItem[]) => {},
    mergeSource = null as MergeContext["source"],
    onMergeStart = (
      _laneId: string,
      _itemId: string,
      _body: string,
      _vote_count: number,
    ) => {},
    onMergeCancel = () => {},
  }: {
    laneId?: string;
    lane?: LaneType;
    items?: DndItem[];
    sendAction?: (...args: any[]) => void;
    onDndConsider?: (items: DndItem[]) => void;
    onDndFinalize?: (items: DndItem[]) => void;
    mergeSource?: MergeContext["source"];
    onMergeStart?: (
      laneId: string,
      itemId: string,
      body: string,
      vote_count: number,
    ) => void;
    onMergeCancel?: () => void;
  } = $props();

  setMergeContext({
    get source() {
      return mergeSource;
    },
    start: onMergeStart,
    cancel: onMergeCancel,
  });
</script>

<Lane {laneId} {lane} {sendAction} {items} {onDndConsider} {onDndFinalize} />

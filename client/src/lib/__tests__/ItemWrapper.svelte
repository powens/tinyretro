<script lang="ts">
  import Item from "../Item.svelte";
  import { setMergeContext, type MergeContext } from "../merge-context";

  let {
    id = "item-1",
    body = "Test item",
    vote_count = 0,
    laneId = "lane-1",
    sendAction = () => {},
    cardBg = "",
    cardBorder = "",
    mergeSource = null as MergeContext["source"],
    onMergeStart = (() => {}) as (
      laneId: string,
      itemId: string,
      body: string,
      vote_count: number,
    ) => void,
    onMergeCancel = () => {},
  }: {
    id?: string;
    body?: string;
    vote_count?: number;
    laneId?: string;
    sendAction?: (...args: unknown[]) => void;
    cardBg?: string;
    cardBorder?: string;
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

<Item {id} {body} {vote_count} {laneId} {sendAction} {cardBg} {cardBorder} />

import { getContext, setContext } from "svelte";
import type { MergeSource } from "$lib/BoardState.svelte";

const MERGE_CTX_KEY = Symbol("merge-context");

export type MergeContext = {
  readonly source: MergeSource;
  start: (
    laneId: string,
    itemId: string,
    body: string,
    vote_count: number,
  ) => void;
  cancel: () => void;
};

export function setMergeContext(ctx: MergeContext) {
  setContext(MERGE_CTX_KEY, ctx);
}

export function getMergeContext(): MergeContext {
  return getContext<MergeContext>(MERGE_CTX_KEY);
}

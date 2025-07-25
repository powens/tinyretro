<script lang="ts">
  import type { Lane } from "./BoardState.svelte";
  import RetroItem from "./RetroItem.svelte";
  import AddItem from "./AddItem.svelte";
  let { laneId, lane }: { laneId: string; lane: Lane } = $props();
</script>

<div>
  <h3 class="title">{lane.title}</h3>
  <div>
    {#each Object.entries(lane.items) as [itemId, item] (itemId)}
      <RetroItem
        body={item.body}
        vote_count={item.vote_count}
        theme={lane.theme}
        {laneId}
        id={itemId}
      />
    {/each}

    <AddItem {laneId} />
  </div>
</div>

<style>
  .title {
    font-size: 2rem;
  }
</style>

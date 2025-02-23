<script lang="ts">
  import type { Board } from "./BoardState.svelte";
  import RetroLane from "./RetroLane.svelte";

  const { boardState }: { boardState: Board } = $props();
</script>

<div class="board">
  {#each Object.entries(boardState.lanes) as [laneId, lane]}
  <div class="lane">
        <RetroLane laneId={laneId} lane={lane} />
    </div>
    {/each}
</div>

<style>
    .board {
        display: grid;
        grid-template-columns: repeat(3, 1fr);
        gap: 1rem;
    }
    .lane {
        /* border: 1px solid; */
        padding: 1rem;
    }
</style>

<script lang="ts">
  import WebsocketWrapper from "../WebsocketWrapper.svelte";
</script>

<WebsocketWrapper>
  {#snippet children({ boardState, sendAction, socketState })}
    <p data-testid="socket-state">{socketState}</p>
    {#if boardState}
      <p data-testid="board-title">{boardState.title}</p>
    {:else}
      <p data-testid="no-board">No board</p>
    {/if}
    <button
      data-testid="send-btn"
      onclick={() =>
        sendAction({ type: "AddItem", lane_id: "l1", body: "test" })}
    >
      Send
    </button>
  {/snippet}
</WebsocketWrapper>

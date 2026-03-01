<script lang="ts">
  import WebsocketWrapper from "$lib/WebsocketWrapper.svelte";
  import Board from "$lib/Board.svelte";
  import { AppBar } from "@skeletonlabs/skeleton-svelte";
</script>

<WebsocketWrapper>
  {#snippet children({ boardState, sendAction, socketState })}
    <div class="min-h-screen bg-surface-50 dark:bg-surface-900">
      <!-- Header -->
      <AppBar>
        <AppBar.Toolbar>
          <AppBar.Lead>
            <span class="text-xl font-bold tracking-tight">tinyretro</span>
          </AppBar.Lead>
          <AppBar.Headline>
            <span class="text-lg font-medium opacity-75">
              {boardState?.title ?? "Loading…"}
            </span>
          </AppBar.Headline>
          <AppBar.Trail>
            {#if socketState === "connected"}
              <span class="badge preset-filled-success-200-800 text-xs">
                ● connected
              </span>
            {:else if socketState === "connecting"}
              <span class="badge preset-filled-warning-200-800 text-xs">
                ◌ connecting
              </span>
            {:else}
              <span class="badge preset-filled-error-200-800 text-xs">
                ○ disconnected
              </span>
            {/if}
          </AppBar.Trail>
        </AppBar.Toolbar>
      </AppBar>

      <!-- Board -->
      <div class="p-4 md:p-8">
        {#if boardState}
          <Board {boardState} {sendAction} />
        {:else}
          <div class="flex items-center justify-center py-20">
            <p class="text-lg opacity-50">
              {socketState === "disconnected"
                ? "Could not connect to server"
                : "Loading board…"}
            </p>
          </div>
        {/if}
      </div>
    </div>
  {/snippet}
</WebsocketWrapper>

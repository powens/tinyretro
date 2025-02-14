<script lang="ts">
  import { ThumbsUp, Check } from "lucide-svelte";
  import { Button } from "kampsy-ui";
  import { getContext } from "svelte";
  import type { ActionUpvoteItem, AllActions, SendActionFunc } from "./BoardState.svelte";
  let { body, vote_count, theme, laneId, id }: { body: string; vote_count: number, theme: string, laneId: string, id: string } = $props();

  let hasVoted = $state(false);
  let sendAction = getContext<() => SendActionFunc>("sendAction");

</script>

<div class="item {theme}">
  <p class="body">{body}</p>
  <div class="footer">
    <Button
      class="upvote"
      type="secondary"
      onclick={() => {
        const action: ActionUpvoteItem = { type: "UpvoteItem", lane_id: laneId, id: id};
        sendAction()(action);


        hasVoted = true;
        setTimeout(() => {
          hasVoted = false;
        }, 5000);
      }}
      prefix={hasVoted ? Check : ThumbsUp}
    >
      <!-- {#if hasVoted}
        <Check class="icon" />
      {:else}
        <ThumbsUp class="icon" />
      {/if} -->
      {vote_count}
    </Button>
  </div>
</div>

<style>
  .item {
    padding: 0.5rem;
    border: 1px solid;
    margin-bottom: 1rem;
  }
  .body {
    margin-bottom: 0.5rem;
  }
  .footer {
    display: flex;
    align-items: flex-end;
    justify-content: end;
  }
  .icon {
    padding-right: 1rem;
  }
  .upvote {
    display: flex;
  }
  .upvote:hover {
    color: red;
  }

  .went-well {
    background-color: #d4edda;
    border-color: #c3e6cb;
    color: #000;
  }

  .to-improve {
    background-color: #f8d7da;
    border-color: #f5c6cb;
    color: #000;
  }

  .action-items {
    background-color: #cce5ff;
    border-color: #b8daff;
    color: #000;
  }
</style>

<script lang="ts">
  import { getContext } from "svelte";
  import { Button, Textarea } from "kampsy-ui";
  import { Cross, Plus, Save } from "lucide-svelte";
  import type { ActionAddItem, SendActionFunc } from "./BoardState.svelte";

  let { laneId }: { laneId: string } = $props();
  let isAdding = $state(false);
  let body = $state("");

  let sendAction = getContext<() => SendActionFunc>("sendAction");
</script>

{#if isAdding}
  <div>
    <form>
      <Textarea placeholder="Enter item body" value={body} />
      <Button
        type="secondary"
        prefix={Cross}
        on:click={() => {
          isAdding = false;
          body = "";
        }}>Cancel</Button
      >
      <Button
        type="primary"
        prefix={Save}
        on:click={() => {
          const action: ActionAddItem = {
            type: "AddItem",
            lane_id: laneId,
            body: "test",
          };
          sendAction()(action);
          isAdding = false;
        }}>Submit</Button
      >
    </form>
  </div>
{:else}
  <Button type="primary" prefix={Plus} on:click={() => (isAdding = true)}
    >Add item</Button
  >
{/if}

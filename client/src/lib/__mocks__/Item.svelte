<script lang="ts">
  let {
    id,
    body,
    vote_count,
  }: {
    id: string;
    body: string;
    vote_count: number;
    laneId?: string;
    sendAction?: (...args: unknown[]) => void;
    cardBg?: string;
    cardBorder?: string;
  } = $props();
</script>

<div data-testid="item-{id}" aria-label="Retro item: {body}">
  <p>{body}</p>
  <span>{vote_count}</span>
</div>

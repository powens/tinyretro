<script lang="ts">
  // Minimal mock icon component for tests - no rest spread to avoid Svelte 5 init bug
  let { size = 16, strokeWidth = 2 }: { size?: number; strokeWidth?: number } =
    $props();
</script>

<svg
  width={size}
  height={size}
  stroke-width={strokeWidth}
  data-testid="mock-icon"
>
  <rect />
</svg>

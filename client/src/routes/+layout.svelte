<script lang="ts">
  import type { Snippet } from "svelte";
  import "../v2.css";
  let { children }: { children: Snippet } = $props();
</script>

<div class="h-screen" data-theme="cerberus">
  {@render children()}
</div>

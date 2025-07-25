<script lang="ts">
  export let className = "";
  export let type = "button";
</script>

<button class={className} {type} on:click>
  <slot />
</button>

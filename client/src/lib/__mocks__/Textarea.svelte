<script lang="ts">
  export let placeholder = "";
  export let value = "";
</script>

<textarea {placeholder} bind:value on:input></textarea>

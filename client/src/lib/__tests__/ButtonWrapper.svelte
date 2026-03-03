<script lang="ts">
  import Button from "../Button.svelte";

  let {
    text = "Click me",
    variant = undefined,
    size = undefined,
    fullWidth = undefined,
    disabled = undefined,
    onclick = undefined,
    title = undefined,
    class: className = undefined,
  }: {
    text?: string;
    variant?: string;
    size?: string;
    fullWidth?: boolean;
    disabled?: boolean;
    onclick?: () => void;
    title?: string;
    class?: string;
  } = $props();
</script>

<Button
  {variant}
  {size}
  {fullWidth}
  {disabled}
  {onclick}
  {title}
  class={className}
>
  {text}
</Button>

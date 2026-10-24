<script lang="ts">
  import type { ComponentProps } from "svelte";
  import Button from "../Button.svelte";

  type ButtonProps = ComponentProps<typeof Button>;

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
    variant?: ButtonProps["variant"];
    size?: ButtonProps["size"];
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

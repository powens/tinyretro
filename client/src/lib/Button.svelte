<script lang="ts">
  import type { Snippet } from "svelte";
  import type { HTMLButtonAttributes } from "svelte/elements";

  type Variant =
    | "filled-primary"
    | "filled-surface"
    | "filled-success"
    | "filled-error"
    | "outlined"
    | "ghost";

  type Size = "sm" | "md";

  interface Props extends HTMLButtonAttributes {
    variant?: Variant;
    size?: Size;
    fullWidth?: boolean;
    children: Snippet;
  }

  const {
    variant = "filled-primary",
    size = "sm",
    fullWidth = false,
    children,
    class: className = "",
    ...rest
  }: Props = $props();

  const variantClasses: Record<Variant, string> = {
    "filled-primary":
      "preset-filled-primary-500 shadow-sm hover:shadow-md hover:scale-[1.02] active:scale-[0.98]",
    "filled-surface":
      "preset-filled-surface-200-800 shadow-sm hover:preset-filled-primary-500 hover:scale-[1.02] active:scale-[0.98]",
    "filled-success": "preset-filled-success-200-800 shadow-sm scale-105",
    "filled-error":
      "preset-filled-error-500 font-bold shadow-md hover:brightness-110 hover:scale-105 active:scale-[0.98]",
    outlined:
      "preset-outlined-surface-200-800 hover:bg-surface-200/50 dark:hover:bg-surface-700/50 active:scale-[0.98]",
    /** Requires a parent element with the Tailwind `group` class to toggle visibility on hover */
    ghost:
      "opacity-0 group-hover:opacity-60 hover:!opacity-100 transition-opacity",
  };

  const sizeClasses: Record<Size, string> = {
    sm: "btn-sm",
    md: "",
  };

  let classes = $derived(
    [
      "btn",
      sizeClasses[size],
      variantClasses[variant],
      "gap-1.5 font-semibold transition-all duration-200",
      fullWidth ? "w-full" : "",
      className,
    ]
      .filter(Boolean)
      .join(" "),
  );
</script>

<button class={classes} {...rest}>
  {@render children()}
</button>

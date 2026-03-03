import { render, screen, cleanup } from "@testing-library/svelte";
import { describe, test, expect, vi, beforeAll } from "vitest";
import userEvent from "@testing-library/user-event";
import ButtonWrapper from "./__tests__/ButtonWrapper.svelte";

// Warm up svelte component rendering in jsdom
beforeAll(() => {
  try {
    render(ButtonWrapper);
  } catch {
    /* ignore first-render init error */
  }
  cleanup();
});

describe("Button", () => {
  test("renders a <button> element", () => {
    render(ButtonWrapper, { text: "Click me" });
    expect(screen.getByRole("button")).toBeInTheDocument();
  });

  test("renders children text", () => {
    render(ButtonWrapper, { text: "Save" });
    expect(screen.getByRole("button")).toHaveTextContent("Save");
  });

  test("applies default variant classes (filled-primary)", () => {
    render(ButtonWrapper);
    const btn = screen.getByRole("button");
    expect(btn.className).toContain("preset-filled-primary-500");
  });

  test("applies filled-surface variant", () => {
    render(ButtonWrapper, { variant: "filled-surface" });
    const btn = screen.getByRole("button");
    expect(btn.className).toContain("preset-filled-surface-200-800");
  });

  test("applies filled-error variant", () => {
    render(ButtonWrapper, { variant: "filled-error" });
    const btn = screen.getByRole("button");
    expect(btn.className).toContain("preset-filled-error-500");
  });

  test("applies filled-success variant", () => {
    render(ButtonWrapper, { variant: "filled-success" });
    const btn = screen.getByRole("button");
    expect(btn.className).toContain("preset-filled-success-200-800");
  });

  test("applies outlined variant", () => {
    render(ButtonWrapper, { variant: "outlined" });
    const btn = screen.getByRole("button");
    expect(btn.className).toContain("preset-outlined-surface-200-800");
  });

  test("applies ghost variant", () => {
    render(ButtonWrapper, { variant: "ghost" });
    const btn = screen.getByRole("button");
    expect(btn.className).toContain("opacity-0");
    expect(btn.className).toContain("group-hover:opacity-60");
  });

  test("applies sm size by default", () => {
    render(ButtonWrapper);
    const btn = screen.getByRole("button");
    expect(btn.className).toContain("btn-sm");
  });

  test("applies md size without btn-sm", () => {
    render(ButtonWrapper, { size: "md" });
    const btn = screen.getByRole("button");
    expect(btn.className).toContain("btn");
    expect(btn.className).not.toContain("btn-sm");
  });

  test("applies fullWidth class when true", () => {
    render(ButtonWrapper, { fullWidth: true });
    const btn = screen.getByRole("button");
    expect(btn.className).toContain("w-full");
  });

  test("does not apply w-full by default", () => {
    render(ButtonWrapper);
    const btn = screen.getByRole("button");
    expect(btn.className).not.toContain("w-full");
  });

  test("passes through disabled attribute", () => {
    render(ButtonWrapper, { disabled: true });
    expect(screen.getByRole("button")).toBeDisabled();
  });

  test("passes through title attribute", () => {
    render(ButtonWrapper, { title: "My tooltip" });
    expect(screen.getByRole("button")).toHaveAttribute("title", "My tooltip");
  });

  test("fires onclick handler", async () => {
    const user = userEvent.setup();
    const onclick = vi.fn();
    render(ButtonWrapper, { onclick });
    await user.click(screen.getByRole("button"));
    expect(onclick).toHaveBeenCalledOnce();
  });

  test("does not fire onclick when disabled", async () => {
    const user = userEvent.setup();
    const onclick = vi.fn();
    render(ButtonWrapper, { onclick, disabled: true });
    await user.click(screen.getByRole("button"));
    expect(onclick).not.toHaveBeenCalled();
  });

  test("always includes base btn class and transition classes", () => {
    render(ButtonWrapper);
    const btn = screen.getByRole("button");
    expect(btn.className).toContain("btn");
    expect(btn.className).toContain("transition-all");
    expect(btn.className).toContain("duration-200");
  });
});

import { render, screen, cleanup } from "@testing-library/svelte";
import { describe, test, expect, vi, beforeAll } from "vitest";
import userEvent from "@testing-library/user-event";
import LaneWrapper from "./__tests__/LaneWrapper.svelte";
import type { DndItem, Lane } from "$lib/BoardState.svelte";

// Mock svelte-dnd-action — use:dndzone relies on pointer events / DOM
// measurements not available in jsdom.
vi.mock("svelte-dnd-action", () => ({
  dndzone: () => ({ destroy() {} }),
  SHADOW_PLACEHOLDER_ITEM_ID: "shadow-placeholder",
}));

// Mock Item to keep Lane tests focused and avoid Svelte 5 jsdom issues.
vi.mock("$lib/Item.svelte", async () => {
  const MockItem = (await import("./__mocks__/Item.svelte")).default;
  return { default: MockItem };
});

// Mock @lucide/svelte (used transitively by Item, though mocked above)
vi.mock("@lucide/svelte", async () => {
  const MockIcon = (await import("./__mocks__/MockIcon.svelte")).default;
  return {
    ThumbsUp: MockIcon,
    Check: MockIcon,
    GripVertical: MockIcon,
    Pencil: MockIcon,
    X: MockIcon,
    Trash2: MockIcon,
    Merge: MockIcon,
  };
});

// Warm up Svelte 5 component rendering in jsdom
beforeAll(() => {
  try {
    render(LaneWrapper);
  } catch {}
  cleanup();
});

const defaultLane: Lane = {
  title: "Went Well",
  theme: "went-well",
  items: {},
};

function makeItems(...bodies: string[]): DndItem[] {
  return bodies.map((body, i) => ({
    id: `item-${i + 1}`,
    body,
    vote_count: 0,
    sort_order: i,
  }));
}

describe("Lane", () => {
  const user = userEvent.setup();

  // -- Header rendering --
  test("renders lane title", () => {
    render(LaneWrapper, { lane: defaultLane });
    expect(screen.getByText("Went Well")).toBeInTheDocument();
  });

  test("renders theme icon", () => {
    render(LaneWrapper, { lane: defaultLane });
    expect(screen.getByText("🎉")).toBeInTheDocument();
  });

  test("renders item count badge", () => {
    const items = makeItems("a", "b", "c");
    render(LaneWrapper, { lane: defaultLane, items });
    expect(screen.getByText("3")).toBeInTheDocument();
  });

  // -- Empty state --
  test("shows empty state when no items", () => {
    render(LaneWrapper, { lane: defaultLane, items: [] });
    expect(screen.getByText("No items yet")).toBeInTheDocument();
    expect(screen.getByText("Click + to add one")).toBeInTheDocument();
  });

  test("does not show empty state when items exist", () => {
    const items = makeItems("something");
    render(LaneWrapper, { lane: defaultLane, items });
    expect(screen.queryByText("No items yet")).not.toBeInTheDocument();
  });

  // -- Item rendering --
  test("renders items", () => {
    const items = makeItems("Task A", "Task B");
    render(LaneWrapper, { lane: defaultLane, items });
    expect(screen.getByText("Task A")).toBeInTheDocument();
    expect(screen.getByText("Task B")).toBeInTheDocument();
  });

  // -- Add item flow --
  test("shows add-item button initially", () => {
    render(LaneWrapper, { lane: defaultLane });
    expect(
      screen.getByRole("button", { name: /add item/i }),
    ).toBeInTheDocument();
  });

  test("opens add form when add-item button clicked", async () => {
    render(LaneWrapper, { lane: defaultLane });
    await user.click(screen.getByRole("button", { name: /add item/i }));
    expect(
      screen.getByPlaceholderText("What's on your mind?"),
    ).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Add" })).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Cancel" })).toBeInTheDocument();
  });

  test("hides empty state when add form is open", async () => {
    render(LaneWrapper, { lane: defaultLane, items: [] });
    await user.click(screen.getByRole("button", { name: /add item/i }));
    expect(screen.queryByText("No items yet")).not.toBeInTheDocument();
  });

  test("sends AddItem action on submit", async () => {
    const sendAction = vi.fn();
    render(LaneWrapper, {
      laneId: "lane-42",
      lane: defaultLane,
      sendAction,
    });
    await user.click(screen.getByRole("button", { name: /add item/i }));
    const textarea = screen.getByPlaceholderText("What's on your mind?");
    await user.type(textarea, "New item body");
    await user.click(screen.getByRole("button", { name: "Add" }));
    expect(sendAction).toHaveBeenCalledWith({
      type: "AddItem",
      lane_id: "lane-42",
      body: "New item body",
    });
  });

  test("sends AddItem on Ctrl+Enter", async () => {
    const sendAction = vi.fn();
    render(LaneWrapper, {
      laneId: "lane-42",
      lane: defaultLane,
      sendAction,
    });
    await user.click(screen.getByRole("button", { name: /add item/i }));
    const textarea = screen.getByPlaceholderText("What's on your mind?");
    await user.type(textarea, "Keyboard submit");
    await user.keyboard("{Control>}{Enter}{/Control}");
    expect(sendAction).toHaveBeenCalledWith({
      type: "AddItem",
      lane_id: "lane-42",
      body: "Keyboard submit",
    });
  });

  test("closes add form and resets on cancel", async () => {
    render(LaneWrapper, { lane: defaultLane });
    await user.click(screen.getByRole("button", { name: /add item/i }));
    const textarea = screen.getByPlaceholderText("What's on your mind?");
    await user.type(textarea, "will be discarded");
    await user.click(screen.getByRole("button", { name: "Cancel" }));
    // Form should close
    expect(
      screen.queryByPlaceholderText("What's on your mind?"),
    ).not.toBeInTheDocument();
    // Add button should reappear
    expect(
      screen.getByRole("button", { name: /add item/i }),
    ).toBeInTheDocument();
  });

  test("closes add form on Escape", async () => {
    render(LaneWrapper, { lane: defaultLane });
    await user.click(screen.getByRole("button", { name: /add item/i }));
    const textarea = screen.getByPlaceholderText("What's on your mind?");
    await user.type(textarea, "escape me");
    await user.keyboard("{Escape}");
    expect(
      screen.queryByPlaceholderText("What's on your mind?"),
    ).not.toBeInTheDocument();
  });

  test("does not send AddItem if body is empty", async () => {
    const sendAction = vi.fn();
    render(LaneWrapper, { lane: defaultLane, sendAction });
    await user.click(screen.getByRole("button", { name: /add item/i }));
    // Click Add without typing
    await user.click(screen.getByRole("button", { name: "Add" }));
    expect(sendAction).not.toHaveBeenCalled();
  });

  test("does not send AddItem if body is whitespace only", async () => {
    const sendAction = vi.fn();
    render(LaneWrapper, { lane: defaultLane, sendAction });
    await user.click(screen.getByRole("button", { name: /add item/i }));
    const textarea = screen.getByPlaceholderText("What's on your mind?");
    await user.type(textarea, "   ");
    await user.click(screen.getByRole("button", { name: "Add" }));
    expect(sendAction).not.toHaveBeenCalled();
  });

  test("Add button is disabled when text is empty", async () => {
    render(LaneWrapper, { lane: defaultLane });
    await user.click(screen.getByRole("button", { name: /add item/i }));
    expect(screen.getByRole("button", { name: "Add" })).toBeDisabled();
  });

  // -- Theme rendering --
  test("renders to-improve theme", () => {
    const lane: Lane = { title: "To Improve", theme: "to-improve", items: {} };
    render(LaneWrapper, { lane });
    expect(screen.getByText("🔧")).toBeInTheDocument();
    expect(screen.getByText("To Improve")).toBeInTheDocument();
  });

  test("renders action-items theme", () => {
    const lane: Lane = {
      title: "Action Items",
      theme: "action-items",
      items: {},
    };
    render(LaneWrapper, { lane });
    expect(screen.getByText("🚀")).toBeInTheDocument();
    expect(screen.getByText("Action Items")).toBeInTheDocument();
  });
});

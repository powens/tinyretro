import { render, screen, cleanup } from "@testing-library/svelte";
import { describe, test, expect, vi } from "vitest";
import userEvent from "@testing-library/user-event";
import ItemWrapper from "./__tests__/ItemWrapper.svelte";

// Mock @lucide/svelte icons — Icon.svelte uses {...props} spread which
// triggers a Svelte 5 first-render bug in jsdom.
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

// Helper: warm up + render. Svelte 5's template hydration in jsdom needs
// several render/cleanup cycles before text-node references stabilise.
function renderItem(props: Record<string, unknown> = {}) {
  cleanup();
  const defaults = {
    id: "item-1",
    body: "Test item",
    vote_count: 0,
    laneId: "lane-1",
  };
  const merged = { ...defaults, ...props };
  for (let i = 0; i < 5; i++) {
    try {
      render(ItemWrapper, merged);
    } catch {
      /* Svelte 5 jsdom warm-up */
    }
    cleanup();
  }
  return render(ItemWrapper, merged);
}

describe("Item", () => {
  const user = userEvent.setup();

  // -- View mode rendering --
  test("renders item body text", () => {
    renderItem({ body: "Great sprint!", vote_count: 3 });
    expect(screen.getByText("Great sprint!")).toBeInTheDocument();
  });

  test("renders vote count", () => {
    renderItem({ vote_count: 7 });
    expect(screen.getByText("7")).toBeInTheDocument();
  });

  test("has aria-label with body", () => {
    renderItem({ body: "Improve CI" });
    expect(screen.getByLabelText("Retro item: Improve CI")).toBeInTheDocument();
  });

  test("renders edit, merge, and delete buttons in view mode", () => {
    renderItem();
    expect(screen.getByTitle("Edit item")).toBeInTheDocument();
    expect(screen.getByTitle("Merge with another item")).toBeInTheDocument();
    expect(screen.getByTitle("Delete item")).toBeInTheDocument();
  });

  // -- Upvote --
  test("sends UpvoteItem action when upvote clicked", async () => {
    const sendAction = vi.fn();
    renderItem({
      id: "i1",
      laneId: "l1",
      vote_count: 2,
      sendAction,
    });
    // The upvote button contains the vote count
    const upvoteBtn = screen.getByText("2").closest("button")!;
    await user.click(upvoteBtn);
    expect(sendAction).toHaveBeenCalledWith({
      type: "UpvoteItem",
      lane_id: "l1",
      id: "i1",
    });
  });

  // -- Edit mode --
  test("enters edit mode on double-click of body text", async () => {
    renderItem({ body: "Edit me" });
    const bodyEl = screen.getByText("Edit me");
    await user.dblClick(bodyEl);
    // Should show a textarea with the current body
    expect(screen.getByRole("textbox")).toHaveValue("Edit me");
  });

  test("enters edit mode when edit button is clicked", async () => {
    renderItem({ body: "Click edit" });
    await user.click(screen.getByTitle("Edit item"));
    expect(screen.getByRole("textbox")).toHaveValue("Click edit");
  });

  test("saves edit and sends EditItem action on Ctrl+Enter", async () => {
    const sendAction = vi.fn();
    renderItem({
      id: "i2",
      laneId: "l2",
      body: "Original",
      sendAction,
    });
    await user.dblClick(screen.getByText("Original"));
    const textarea = screen.getByRole("textbox");
    await user.clear(textarea);
    await user.type(textarea, "Updated text");
    await user.keyboard("{Control>}{Enter}{/Control}");
    expect(sendAction).toHaveBeenCalledWith({
      type: "EditItem",
      lane_id: "l2",
      id: "i2",
      body: "Updated text",
    });
  });

  test("cancels edit on Escape without sending action", async () => {
    const sendAction = vi.fn();
    renderItem({ body: "No change", sendAction });
    await user.dblClick(screen.getByText("No change"));
    const textarea = screen.getByRole("textbox");
    expect(textarea).toBeInTheDocument();
    // Focus textarea explicitly (requestAnimationFrame doesn't fire in jsdom)
    textarea.focus();
    await user.keyboard("{Escape}");
    // Should return to view mode showing original text
    expect(screen.getByText("No change")).toBeInTheDocument();
    expect(screen.queryByRole("textbox")).not.toBeInTheDocument();
    expect(sendAction).not.toHaveBeenCalled();
  });

  test("does not send EditItem if text unchanged", async () => {
    const sendAction = vi.fn();
    renderItem({ body: "Same text", sendAction });
    await user.dblClick(screen.getByText("Same text"));
    // Press Ctrl+Enter without changing text
    await user.keyboard("{Control>}{Enter}{/Control}");
    expect(sendAction).not.toHaveBeenCalled();
  });

  test("does not send EditItem if text is only whitespace", async () => {
    const sendAction = vi.fn();
    renderItem({ body: "Will clear", sendAction });
    await user.dblClick(screen.getByText("Will clear"));
    const textarea = screen.getByRole("textbox");
    await user.clear(textarea);
    await user.type(textarea, "   ");
    await user.keyboard("{Control>}{Enter}{/Control}");
    expect(sendAction).not.toHaveBeenCalled();
  });

  // -- Delete confirmation --
  test("shows delete confirmation when delete button clicked", async () => {
    renderItem({ body: "Delete me" });
    await user.click(screen.getByTitle("Delete item"));
    expect(screen.getByText("Delete this item?")).toBeInTheDocument();
    expect(
      screen.getByText("This action cannot be undone."),
    ).toBeInTheDocument();
  });

  test("sends RemoveItem action when delete is confirmed", async () => {
    const sendAction = vi.fn();
    renderItem({
      id: "i3",
      laneId: "l3",
      body: "Delete me",
      sendAction,
    });
    await user.click(screen.getByTitle("Delete item"));
    // Click the "Delete" confirmation button
    const deleteBtn = screen.getByRole("button", { name: /Delete/i });
    await user.click(deleteBtn);
    expect(sendAction).toHaveBeenCalledWith({
      type: "RemoveItem",
      lane_id: "l3",
      id: "i3",
    });
  });

  test("cancels delete confirmation and returns to view mode", async () => {
    const sendAction = vi.fn();
    renderItem({ body: "Keep me", sendAction });
    await user.click(screen.getByTitle("Delete item"));
    expect(screen.getByText("Delete this item?")).toBeInTheDocument();
    // Click "Cancel"
    await user.click(screen.getByRole("button", { name: "Cancel" }));
    // Should return to view mode
    expect(screen.getByText("Keep me")).toBeInTheDocument();
    expect(screen.queryByText("Delete this item?")).not.toBeInTheDocument();
    expect(sendAction).not.toHaveBeenCalled();
  });

  // -- Merge source state --
  test("shows merge source overlay when item is the merge source", () => {
    renderItem({
      id: "i5",
      laneId: "l5",
      mergeSource: {
        laneId: "l5",
        itemId: "i5",
        body: "Source item",
        vote_count: 1,
      },
    });
    expect(screen.getByText("Merge source selected")).toBeInTheDocument();
    expect(
      screen.getByText("Click another item in this lane to merge into it."),
    ).toBeInTheDocument();
  });

  test("calls merge.cancel when cancel button clicked in merge source mode", async () => {
    const onMergeCancel = vi.fn();
    renderItem({
      id: "i5",
      laneId: "l5",
      mergeSource: {
        laneId: "l5",
        itemId: "i5",
        body: "Source",
        vote_count: 1,
      },
      onMergeCancel,
    });
    await user.click(screen.getByRole("button", { name: /Cancel merge/i }));
    expect(onMergeCancel).toHaveBeenCalled();
  });

  // -- Merge target state --
  test("shows 'Merge into this' when item is a merge target", () => {
    renderItem({
      id: "target-1",
      laneId: "l5",
      body: "Target item",
      vote_count: 3,
      mergeSource: {
        laneId: "l5",
        itemId: "other-item",
        body: "Source",
        vote_count: 1,
      },
    });
    expect(screen.getByText("Merge into this")).toBeInTheDocument();
  });

  test("opens merge editor when merge target is clicked", async () => {
    renderItem({
      id: "target-2",
      laneId: "l5",
      body: "Target body",
      vote_count: 2,
      mergeSource: {
        laneId: "l5",
        itemId: "source-1",
        body: "Source body",
        vote_count: 1,
      },
    });
    // Click the merge target button
    await user.click(screen.getByText("Merge into this").closest("button")!);
    // Should show the merge editing overlay
    expect(screen.getByText("Merge items")).toBeInTheDocument();
    // Textarea should contain combined text
    const textarea = screen.getByRole("textbox");
    expect(textarea).toHaveValue("Source body\n\nTarget body");
  });

  test("sends MergeItems action when merge is confirmed", async () => {
    const sendAction = vi.fn();
    const onMergeCancel = vi.fn();
    renderItem({
      id: "target-3",
      laneId: "l5",
      body: "Target text",
      vote_count: 2,
      sendAction,
      onMergeCancel,
      mergeSource: {
        laneId: "l5",
        itemId: "source-3",
        body: "Source text",
        vote_count: 1,
      },
    });
    await user.click(screen.getByText("Merge into this").closest("button")!);
    // Focus merge textarea explicitly (requestAnimationFrame doesn't fire in jsdom)
    const textarea = screen.getByRole("textbox");
    textarea.focus();
    // Confirm via Ctrl+Enter
    await user.keyboard("{Control>}{Enter}{/Control}");
    expect(sendAction).toHaveBeenCalledWith({
      type: "MergeItems",
      lane_id: "l5",
      source_id: "source-3",
      target_id: "target-3",
      merged_body: "Source text\n\nTarget text",
    });
    expect(onMergeCancel).toHaveBeenCalled();
  });

  test("calls merge.start when merge button clicked", async () => {
    const onMergeStart = vi.fn();
    renderItem({
      id: "i10",
      laneId: "l10",
      body: "Merge me",
      vote_count: 5,
      onMergeStart,
    });
    await user.click(screen.getByTitle("Merge with another item"));
    expect(onMergeStart).toHaveBeenCalledWith("l10", "i10", "Merge me", 5);
  });
});

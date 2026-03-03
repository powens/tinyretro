import { render, screen, cleanup } from "@testing-library/svelte";
import { describe, test, expect, vi } from "vitest";
import Board from "./Board.svelte";
import type { Board as BoardType } from "$lib/BoardState.svelte";

// Mock svelte-dnd-action
vi.mock("svelte-dnd-action", () => ({
  dndzone: () => ({ destroy() {} }),
  SHADOW_PLACEHOLDER_ITEM_ID: "shadow-placeholder",
  SHADOW_ITEM_MARKER_PROPERTY_NAME: "isDndShadowItem",
}));

// Mock Item to avoid Svelte 5 jsdom issues
vi.mock("$lib/Item.svelte", async () => {
  const MockItem = (await import("./__mocks__/Item.svelte")).default;
  return { default: MockItem };
});

// Mock @lucide/svelte
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

// Warm up + render helper (Svelte 5 jsdom first-render bug)
function renderBoard(props: {
  boardState: BoardType;
  sendAction?: (...args: unknown[]) => void;
}) {
  cleanup();
  const merged = { sendAction: () => {}, ...props };
  for (let i = 0; i < 3; i++) {
    try {
      render(Board, merged);
    } catch {
      /* Svelte 5 jsdom warm-up */
    }
    cleanup();
  }
  return render(Board, merged);
}

function makeBoard(overrides: Partial<BoardType> = {}): BoardType {
  return {
    title: "Retro Board",
    lanes: {
      "lane-1": {
        title: "Went Well",
        theme: "went-well",
        items: {
          "item-1": { body: "Good teamwork", vote_count: 3, sort_order: 0 },
          "item-2": { body: "On time delivery", vote_count: 1, sort_order: 1 },
        },
      },
      "lane-2": {
        title: "To Improve",
        theme: "to-improve",
        items: {
          "item-3": { body: "More testing", vote_count: 2, sort_order: 0 },
        },
      },
      "lane-3": {
        title: "Action Items",
        theme: "action-items",
        items: {},
      },
    },
    ...overrides,
  };
}

describe("Board", () => {
  test("renders all lane titles", () => {
    renderBoard({ boardState: makeBoard() });
    expect(screen.getByText("Went Well")).toBeInTheDocument();
    expect(screen.getByText("To Improve")).toBeInTheDocument();
    expect(screen.getByText("Action Items")).toBeInTheDocument();
  });

  test("renders items inside lanes", () => {
    renderBoard({ boardState: makeBoard() });
    expect(screen.getByText("Good teamwork")).toBeInTheDocument();
    expect(screen.getByText("On time delivery")).toBeInTheDocument();
    expect(screen.getByText("More testing")).toBeInTheDocument();
  });

  test("renders empty state for lane with no items", () => {
    renderBoard({ boardState: makeBoard() });
    expect(screen.getByText("No items yet")).toBeInTheDocument();
  });

  test("renders theme icons for each lane", () => {
    renderBoard({ boardState: makeBoard() });
    expect(screen.getByText("🎉")).toBeInTheDocument();
    expect(screen.getByText("🔧")).toBeInTheDocument();
    expect(screen.getByText("🚀")).toBeInTheDocument();
  });

  test("renders with empty lanes object", () => {
    renderBoard({ boardState: makeBoard({ lanes: {} }) });
    expect(screen.queryByText("Went Well")).not.toBeInTheDocument();
  });

  test("renders item count badges", () => {
    renderBoard({ boardState: makeBoard() });
    // Badge elements have the "badge" class
    const badges = screen
      .getAllByText(/^\d+$/)
      .filter(
        (el) => el.classList.contains("badge") || el.closest(".badge") !== null,
      );
    const badgeTexts = badges.map((b) => b.textContent).sort();
    expect(badgeTexts).toEqual(["0", "1", "2"]);
  });
});

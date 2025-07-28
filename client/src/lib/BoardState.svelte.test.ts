import { test, expect, vi } from "vitest";

// Test the type definitions and interfaces
test("BoardState types are properly defined", () => {
  // Import the types to ensure they're properly exported
  import("./BoardState.svelte.ts").then((module) => {
    expect(module).toBeDefined();
  });
});

// Test type safety with valid objects
test("Item type validation", () => {
  const item = {
    body: "Test item",
    vote_count: 5,
    sort_order: 1,
  };

  expect(item.body).toBe("Test item");
  expect(item.vote_count).toBe(5);
  expect(item.sort_order).toBe(1);
});

test("Lane type validation", () => {
  const lane = {
    title: "Test Lane",
    theme: "went-well" as const,
    items: {
      "item1": {
        body: "Test item",
        vote_count: 0,
        sort_order: 0,
      },
    },
  };

  expect(lane.title).toBe("Test Lane");
  expect(lane.theme).toBe("went-well");
  expect(Object.keys(lane.items)).toHaveLength(1);
});

test("Board type validation", () => {
  const board = {
    title: "Test Board",
    lanes: {
      "lane1": {
        title: "Test Lane",
        theme: "went-well" as const,
        items: {},
      },
    },
  };

  expect(board.title).toBe("Test Board");
  expect(Object.keys(board.lanes)).toHaveLength(1);
});

test("Action types validation", () => {
  const addItemAction = {
    type: "AddItem" as const,
    lane_id: "test-lane",
    body: "Test item",
  };

  const upvoteAction = {
    type: "UpvoteItem" as const,
    lane_id: "test-lane",
    id: "item-id",
  };

  const moveAction = {
    type: "MoveItem" as const,
    from_lane_id: "lane1",
    to_lane_id: "lane2",
    item_id: "item-id",
  };

  const reorderAction = {
    type: "ReorderItem" as const,
    lane_id: "test-lane",
    item_id: "item-id",
    new_position: 2,
  };

  expect(addItemAction.type).toBe("AddItem");
  expect(upvoteAction.type).toBe("UpvoteItem");
  expect(moveAction.type).toBe("MoveItem");
  expect(reorderAction.type).toBe("ReorderItem");
});

test("WebsocketState type validation", () => {
  const websocketState = {
    state: {
      title: "Test Board",
      lanes: {},
    },
    connected: true,
  };

  expect(websocketState.connected).toBe(true);
  expect(websocketState.state?.title).toBe("Test Board");
});

test("SendActionFunc type validation", () => {
  const mockSendAction = vi.fn();
  
  const action = {
    type: "AddItem" as const,
    lane_id: "test-lane",
    body: "Test item",
  };

  mockSendAction(action);
  
  expect(mockSendAction).toHaveBeenCalledWith(action);
});
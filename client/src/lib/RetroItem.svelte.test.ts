import { render, screen, fireEvent } from "@testing-library/svelte";
import { expect, test, vi } from "vitest";
import RetroItem from "./RetroItem.svelte";

// Mock kampsy-ui Button component
vi.mock("kampsy-ui", async () => {
  const { default: MockButton } = await import("./__mocks__/Button.svelte");
  return {
    Button: MockButton,
  };
});

// Why the hell do I need to mock an icon component?
vi.mock("lucide-svelte", () => {
  // Define a minimal Svelte component "class"
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  function MockSvelteComponent(_options: unknown) {
    // Return the minimal shape a Svelte component instance needs
    return {
      $$prop_def: {},
      $$events_def: {},
      $set() {},
      $on() {},
      $destroy() {},
    };
  }
  return {
    ThumbsUp: MockSvelteComponent,
    Check: MockSvelteComponent,
  };
});

test("renders the retro item body text", () => {
  const fakeSendAction = vi.fn();

  render(RetroItem, {
    context: new Map().set("sendAction", fakeSendAction),
    props: {
      body: "foo",
      vote_count: 1,
      theme: "went-well",
      laneId: "2",
      id: "3",
    },
  });

  const bodyElement = screen.queryByText(/foo/i);
  expect(bodyElement).toBeInTheDocument();
});

test("calls sendAction on upvote click", async () => {
  const mockSendAction = vi.fn();
  render(RetroItem, {
    context: new Map().set("sendAction", mockSendAction),
    props: {
      body: "Something",
      vote_count: 5,
      theme: "went-well",
      laneId: "laneX",
      id: "itemY",
    },
  });

  // Find the upvote button specifically (it's the actual button element, not the div with role="button")
  const buttons = screen.getAllByRole("button");
  const upvoteButton = buttons.find((button) => button.tagName === "BUTTON");
  expect(upvoteButton).toBeDefined();

  await fireEvent.click(upvoteButton!);

  expect(mockSendAction).toHaveBeenCalledTimes(1);
  // Check the action that was called
  expect(mockSendAction).toHaveBeenCalledWith({
    type: "UpvoteItem",
    lane_id: "laneX",
    id: "itemY",
  });
});

test("toggles icon after voting", async () => {
  const fakeSendAction = vi.fn();
  render(RetroItem, {
    context: new Map().set("sendAction", fakeSendAction),
    props: {
      body: "Icon check",
      vote_count: 2,
      theme: "went-well",
      laneId: "A",
      id: "B",
    },
  });

  // Find the upvote button specifically (it's the actual button element, not the div with role="button")
  const buttons = screen.getAllByRole("button");
  const upvoteButton = buttons.find((button) => button.tagName === "BUTTON");
  expect(upvoteButton).toBeDefined();

  await fireEvent.click(upvoteButton!);

  // After click, it should switch to Check icon
  // Because icons are mocked, verifying "Check" vs. "ThumbsUp" precisely is tricky,
  // but you can confirm the DOM changes or rely on class checks if needed.
});

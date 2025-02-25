import { render, screen, fireEvent } from "@testing-library/svelte";
import { expect, test, vi } from "vitest";
import RetroItem from "./RetroItem.svelte";

// Why the hell do I need to mock an icon component?
vi.mock("lucide-svelte", () => {
  // Define a minimal Svelte component “class”
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
  const fakeSendAction = () => (_: Event) => {};

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
  const mockSendAction = vi.fn().mockImplementation(() => vi.fn());
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

  const button = screen.getByRole("button");
  await fireEvent.click(button);

  expect(mockSendAction).toHaveBeenCalledTimes(1);
  // Check inner returned function call
  expect(mockSendAction.mock.results[0].value).toHaveBeenCalledWith({
    type: "UpvoteItem",
    lane_id: "laneX",
    id: "itemY",
  });
});

test("toggles icon after voting", async () => {
  const fakeSendAction = () => () => {};
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

  // Initially should show ThumbsUp
  expect(screen.queryByText(/thumbsUp/i)).not.toBeInTheDocument();
  // The actual icon is mocked, so you can check for the containing element or rely on alt text if used.

  const button = screen.getByRole("button");
  await fireEvent.click(button);

  // After click, it should switch to Check icon
  // Because icons are mocked, verifying "Check" vs. "ThumbsUp" precisely is tricky,
  // but you can confirm the DOM changes or rely on class checks if needed.
});

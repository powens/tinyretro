import { render, screen } from "@testing-library/svelte";
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

test("renders a retro item", async () => {
  const fakeSendAction = () => {
    console.log("sendAction called");
    return (action: Event) => {
      console.log("Inner action function called with", action);
    };
  };

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

  const body = screen.queryByText(/foo/iu);

  expect(body).toBeInTheDocument();
});

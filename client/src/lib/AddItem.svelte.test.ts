import { render, screen, fireEvent } from "@testing-library/svelte";
import { test, expect, vi } from "vitest";
import AddItem from "./AddItem.svelte";

// Mock kampsy-ui components
vi.mock("kampsy-ui", async () => {
  const { default: MockButton } = await import("./__mocks__/Button.svelte");
  const { default: MockTextarea } = await import("./__mocks__/Textarea.svelte");

  return {
    Button: MockButton,
    Textarea: MockTextarea,
  };
});

// Mock the lucide-svelte icons so they don't break in Node
vi.mock("lucide-svelte", () => {
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  function MockIcon(_options: unknown) {
    return {
      $$prop_def: {},
      $$events_def: {},
      $set() {},
      $on() {},
      $destroy() {},
    };
  }
  return {
    Cross: MockIcon,
    Plus: MockIcon,
    Save: MockIcon,
  };
});

test("shows the 'Add item' button by default", () => {
  render(AddItem, {
    props: { laneId: "laneA" },
  });

  expect(screen.getByText("Add item")).toBeInTheDocument();
  expect(screen.queryByText("Submit")).not.toBeInTheDocument();
  expect(screen.queryByText("Cancel")).not.toBeInTheDocument();
});

test("clicking 'Add item' displays the form", async () => {
  render(AddItem, {
    props: { laneId: "laneA" },
  });

  const addItemButton = screen.getByText("Add item");
  await fireEvent.click(addItemButton);

  //   expect(screen.queryByText("Add item")).not.toBeInTheDocument();
  expect(screen.getByText("Cancel")).toBeInTheDocument();
  expect(screen.getByText("Submit")).toBeInTheDocument();
});

test("clicking 'Cancel' hides the form again", async () => {
  render(AddItem, {
    props: { laneId: "laneA" },
  });

  // Show the form
  await fireEvent.click(screen.getByText("Add item"));
  // Click Cancel
  await fireEvent.click(screen.getByText("Cancel"));

  expect(screen.getAllByText("Add item")[0]).toBeInTheDocument();
  expect(screen.queryByText("Submit")).not.toBeInTheDocument();
});

test("clicking 'Submit' calls sendAction with expected arguments", async () => {
  // Mock sendAction so we can verify calls
  const mockSendAction = vi.fn().mockImplementation(() => vi.fn());

  render(AddItem, {
    context: new Map().set("sendAction", mockSendAction),
    props: { laneId: "laneA" },
  });

  // Show the form
  await fireEvent.click(screen.getByText("Add item"));

  // Click Submit
  await fireEvent.click(screen.getByText("Submit"));

  // sendAction was called once, returning a function
  expect(mockSendAction).toHaveBeenCalledTimes(1);

  // That returned function is called with the add-item action:
  expect(mockSendAction.mock.results[0].value).toHaveBeenCalledWith({
    type: "AddItem",
    lane_id: "laneA",
    body: "test",
  });
});

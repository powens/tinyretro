import { render, screen, cleanup } from "@testing-library/svelte";
import { describe, test, expect, vi, beforeEach, afterEach } from "vitest";
import userEvent from "@testing-library/user-event";
import WebsocketWrapperTest from "./__tests__/WebsocketWrapperTest.svelte";

// -- Mock WebSocket --
type WsListener = (event: unknown) => void;

class MockWebSocket {
  static OPEN = 1;
  static instances: MockWebSocket[] = [];

  url: string;
  readyState = 0; // CONNECTING
  listeners: Record<string, WsListener[]> = {};
  sentMessages: string[] = [];

  constructor(url: string) {
    this.url = url;
    MockWebSocket.instances.push(this);
  }

  addEventListener(type: string, listener: WsListener) {
    (this.listeners[type] ??= []).push(listener);
  }

  removeEventListener(type: string, listener: WsListener) {
    const list = this.listeners[type];
    if (list) {
      this.listeners[type] = list.filter((l) => l !== listener);
    }
  }

  send(data: string) {
    this.sentMessages.push(data);
  }

  close() {
    this.readyState = 3; // CLOSED
  }

  // Test helpers
  _emit(type: string, data?: unknown) {
    for (const fn of this.listeners[type] ?? []) {
      fn(data ?? {});
    }
  }

  _open() {
    this.readyState = MockWebSocket.OPEN;
    this._emit("open");
  }

  _message(payload: unknown) {
    this._emit("message", { data: JSON.stringify(payload) });
  }

  _close() {
    this.readyState = 3;
    this._emit("close");
  }
}

// Warm up + render helper
function renderWrapper() {
  cleanup();
  for (let i = 0; i < 3; i++) {
    try {
      render(WebsocketWrapperTest);
    } catch {
      /* Svelte 5 jsdom warm-up */
    }
    cleanup();
  }
  return render(WebsocketWrapperTest);
}

describe("WebsocketWrapper", () => {
  const user = userEvent.setup();

  beforeEach(() => {
    MockWebSocket.instances = [];
    vi.stubGlobal("WebSocket", MockWebSocket);
  });

  afterEach(() => {
    cleanup();
    vi.restoreAllMocks();
  });

  test("starts with connecting state", () => {
    renderWrapper();
    expect(screen.getByTestId("socket-state")).toHaveTextContent(
      "connecting",
    );
  });

  test("shows no board initially", () => {
    renderWrapper();
    expect(screen.getByTestId("no-board")).toBeInTheDocument();
  });

  test("creates WebSocket on mount", () => {
    renderWrapper();
    expect(MockWebSocket.instances.length).toBeGreaterThan(0);
    const ws = MockWebSocket.instances.at(-1)!;
    expect(ws.url).toMatch(/^ws:\/\/.+\/ws$/);
  });

  test("updates to connected when socket opens", async () => {
    renderWrapper();
    const ws = MockWebSocket.instances.at(-1)!;
    ws._open();
    // Wait for Svelte reactivity
    await vi.waitFor(() => {
      expect(screen.getByTestId("socket-state")).toHaveTextContent("connected");
    });
  });

  test("updates boardState when message received", async () => {
    renderWrapper();
    const ws = MockWebSocket.instances.at(-1)!;
    ws._open();
    ws._message({ title: "My Retro", lanes: {} });
    await vi.waitFor(() => {
      expect(screen.getByTestId("board-title")).toHaveTextContent("My Retro");
    });
  });

  test("sends action as JSON via socket", async () => {
    renderWrapper();
    const ws = MockWebSocket.instances.at(-1)!;
    ws._open();
    await vi.waitFor(() => {
      expect(screen.getByTestId("socket-state")).toHaveTextContent("connected");
    });
    await user.click(screen.getByTestId("send-btn"));
    expect(ws.sentMessages).toHaveLength(1);
    expect(JSON.parse(ws.sentMessages[0])).toEqual({
      type: "AddItem",
      lane_id: "l1",
      body: "test",
    });
  });

  test("does not send when socket is not open", async () => {
    renderWrapper();
    const ws = MockWebSocket.instances.at(-1)!;
    // Don't open the socket, readyState stays 0 (CONNECTING)
    // The sendAction won't be initialized until onMount runs, but let's
    // try clicking — the default sendAction logs an error
    await user.click(screen.getByTestId("send-btn"));
    expect(ws.sentMessages).toHaveLength(0);
  });

  test("updates to disconnected when socket closes", async () => {
    renderWrapper();
    const ws = MockWebSocket.instances.at(-1)!;
    ws._open();
    await vi.waitFor(() => {
      expect(screen.getByTestId("socket-state")).toHaveTextContent("connected");
    });
    ws._close();
    await vi.waitFor(() => {
      expect(screen.getByTestId("socket-state")).toHaveTextContent(
        "disconnected",
      );
    });
  });
});

<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import type {
    AllActions,
    Board,
    SendActionFunc,
  } from "../lib/BoardState.svelte";
  import type { Snippet } from "svelte";

  let {
    children,
  }: {
    children: Snippet<
      [
        {
          boardState: Board | undefined;
          sendAction: SendActionFunc;
          socketState: string;
        },
      ]
    >;
  } = $props();

  let socketState = $state("disconnected");
  let sendAction = $state<SendActionFunc>(() => {
    console.error("sendAction not initialized");
  });

  let boardState: Board | undefined = $state(undefined);

  let socket = $state<WebSocket>();

  onMount(() => {
    const hostProtocol = window.location.protocol === "https:" ? "wss" : "ws";
    const hostAddress = window.location.hostname;
    let hostPort = window.location.port ? `:${window.location.port}` : "";
    if (import.meta.env.DEBUG) {
      hostPort = ":3000";
    }

    socket = new WebSocket(`${hostProtocol}://${hostAddress}${hostPort}/ws`);

    socket.addEventListener("open", () => {
      console.debug("Connected to server");
      socketState = "connected";
    });

    socket.addEventListener("close", () => {
      console.debug("Disconnected from server");
      socketState = "disconnected";
    });

    socket.addEventListener("error", (event) => {
      console.error("Error", event);
    });

    socket.addEventListener("message", (event) => {
      console.debug("Message from server", event.data);
      try {
        boardState = JSON.parse(event.data);
      } catch (error) {
        console.error("Error parsing JSON", error);
      }
    });

    sendAction = (action: AllActions) => {
      if (!socket) {
        console.error("Socket not initialized");
        return;
      }
      if (socket.readyState !== WebSocket.OPEN) {
        console.error("Socket not open");
        return;
      }
      console.debug("Sending message", action);
      socket.send(JSON.stringify(action));
    };
  });

  onDestroy(() => {
    if (!socket) {
      return;
    }
    socket.close();
    socketState = "disconnected";
  });
</script>

<div>
  {@render children({ boardState, sendAction, socketState })}
</div>

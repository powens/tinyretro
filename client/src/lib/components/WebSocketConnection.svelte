<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import type { Board, AllActions } from "$lib/BoardState.svelte";

  // Props interface
  interface Props {
    boardState: Board;
    socketState: "connected" | "disconnected" | "connecting";
    sendAction: (action: AllActions) => void;
    hasInitialData: boolean;
  }

  let {
    boardState = $bindable(),
    socketState = $bindable(),
    sendAction = $bindable(),
    hasInitialData = $bindable(),
  }: Props = $props();

  let socket = $state<WebSocket>();

  onMount(() => {
    // Initialize WebSocket connection
    socketState = "connecting";

    const hostProtocol = window.location.protocol === "https:" ? "wss" : "ws";
    const hostAddress = window.location.hostname;
    let hostPort = window.location.port ? `:${window.location.port}` : "";
    if (import.meta.env.DEV) {
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
      console.error("WebSocket error", event);
      socketState = "disconnected";
    });

    socket.addEventListener("message", (event) => {
      console.debug("Message from server", event.data);
      try {
        boardState = JSON.parse(event.data);
        hasInitialData = true;
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
      console.debug("Sending action", action);
      socket.send(JSON.stringify(action));
    };
  });

  onDestroy(() => {
    if (socket) {
      socket.close();
      socketState = "disconnected";
    }
  });
</script>

<!-- This component is purely for WebSocket management and doesn't render anything -->

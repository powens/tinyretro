<script lang="ts">
  // Props
  interface Props {
    socketState: "connected" | "disconnected" | "connecting";
  }

  let { socketState }: Props = $props();

  // Connection status messages
  const statusMessages = {
    connecting: "Connecting to server...",
    connected: "Loading board data...",
    disconnected: "Connection lost. Retrying...",
  };
</script>

<div
  class="min-h-screen bg-gray-50 dark:bg-gray-900 flex items-center justify-center"
>
  <div class="text-center space-y-8">
    <!-- Animated spinner -->
    <div class="relative">
      <div class="w-16 h-16 mx-auto">
        <div
          class="absolute inset-0 border-4 border-gray-200 dark:border-gray-700 rounded-full"
        ></div>
        <div
          class="absolute inset-0 border-4 border-blue-500 border-t-transparent rounded-full animate-spin"
        ></div>
      </div>
    </div>

    <!-- Status message -->
    <div class="space-y-2">
      <h2 class="text-xl font-semibold text-gray-900 dark:text-white">
        Sprint Retrospective Board
      </h2>
      <p class="text-gray-600 dark:text-gray-400">
        {statusMessages[socketState]}
      </p>
    </div>

    <!-- Connection indicator -->
    <div class="flex items-center justify-center space-x-2">
      <div
        class="w-2 h-2 rounded-full {socketState === 'connected'
          ? 'bg-green-500'
          : socketState === 'connecting'
            ? 'bg-yellow-500 animate-pulse'
            : 'bg-red-500'}"
      ></div>
      <span class="text-sm text-gray-500 dark:text-gray-400 capitalize">
        {socketState}
      </span>
    </div>

    <!-- Additional loading dots for connecting state -->
    {#if socketState === "connecting"}
      <div class="flex justify-center space-x-1">
        <div class="w-2 h-2 bg-blue-500 rounded-full animate-bounce"></div>
        <div
          class="w-2 h-2 bg-blue-500 rounded-full animate-bounce"
          style="animation-delay: 0.1s"
        ></div>
        <div
          class="w-2 h-2 bg-blue-500 rounded-full animate-bounce"
          style="animation-delay: 0.2s"
        ></div>
      </div>
    {/if}
  </div>
</div>

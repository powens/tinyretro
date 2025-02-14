<script lang="ts">
  import type { Board } from "$lib/BoardState.svelte";
  import RetroBoard from "$lib/RetroBoard.svelte";
  import RetroItem from "$lib/RetroItem.svelte";
  import WebsocketWrapper from "../lib/WebsocketWrapper.svelte";
  // import { USE_MOCKED_DATA } from "$env/static/private";


  import { ThemeSwitcher } from "kampsy-ui";

  // if (USE_MOCKED_DATA === "true") {

  //   const boardState: Board = {
  //     title: "My board",
  //     lanes: {
  //       "lane-1": {
  //         title: "Went Well",
  //         theme: "went-well",
  //         items: {
  //           "item-1": {
  //             body: "Body 1",
  //             vote_count: 0,
  //             sort_order: 0,
  //           },
  //           "item-2": {
  //             body: "Tihs is a very long body. Probably more and more and more words are needed",
  //             vote_count: 5,
  //             sort_order: 1,
  //           },
  //         },
  //       },
  //       "lane-2": {
  //         title: "To Improve",
  //         theme: "to-improve",
  //         items: {
  //           "item-3": {
  //             body: "Item 3",
  //             vote_count: 100,
  //             sort_order: 0,
  //           },
  //         },
  //       },
  //       "lane-3": {
  //         title: "Action Items",
  //         theme: "action-items",
  //         items: {},
  //       },
  //     },
    // };
  // }
</script>

<header>
  <h1 class="title">tinyretro</h1>
  <div class="w-full h-full">
    <ThemeSwitcher />
  </div>
</header>
<main>
  <WebsocketWrapper />
  <!-- <RetroBoard boardState={boardState} /> -->
</main>

<!-- <div class="ui-scrollbar font-inter min-h-screen overflow-x-hidden md:overflow-x-visible bg-kui-light-bg-secondary dark:bg-kui-dark-bg-secondary">

  <header class="z-50 fixed top-0 mx-auto w-full max-w-[1220px] h-[64px] md:sticky bg-kui-light-bg dark:bg-kui-dark-bg">
    <div class=" bg-kui-light-bg dark:bg-kui-dark-bg w-full h-[64px] flex border-b border-r border-kui-light-gray-200 dark:border-kui-dark-gray-400">
      <div class="hidden lg:block w-full max-w-[260px] h-full border-l border-r border-kui-light-gray-200 dark:border-kui-dark-gray-400 font-size-xl">
        tinyretro
      </div>
      <div class="w-full h-full">
  
        <ThemeSwitcher />
      </div>
    </div>
  
  </header>
  <main>
    <WebsocketWrapper>
      <h1 class="text-3xl font-bold underline">Hello world!</h1>
    </WebsocketWrapper>
    <RetroItem body="hello" vote_count={1} />
  
  </main>
</div> -->

<style>
  @reference "tailwindcss/theme";

  header {
    padding: 1rem;
    display: flex;
  }
  .title {
    font-size: 2rem;
    width: 300px;
  }
</style>

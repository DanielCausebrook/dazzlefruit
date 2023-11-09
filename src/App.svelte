<script lang="ts">
  import Connect from "./lib/Connect.svelte";
  import Debug from "./lib/Debug.svelte";
  import ConnectionBar from "./lib/ConnectionBar.svelte";
  import NeopixelController from "./lib/NeopixelController.svelte";
  import {onDestroy, onMount} from "svelte";
  import {listen} from "@tauri-apps/api/event";
  import PatternBuilder from "./lib/PatternBuilder.svelte";

  let connection: {ip: string} | null = null;
  let unlistenOpen, unlistenClose;

  onMount(async () => {
    unlistenOpen = await listen('connection-open', (event: Event<{ip: string}>) => {
      connection = event.payload;
    });
    unlistenOpen = await listen('connection-close', (event: Event<{}>) => {
      connection = null;
    });
  });

  onDestroy(async () => {
    unlistenOpen();
    unlistenClose();
  });


</script>
<div class="df-app">
  <main class="df-main">
    <PatternBuilder />
  </main>
  <ConnectionBar connection={connection} />
</div>

<style>
  .df-app {
    height: 100vh;
    display: flex;
    flex-flow: column nowrap;
    overflow: clip;
    /*border: 1px solid red;*/
  }
  .df-main {
    flex: 1 1 auto;
    display: flex;
    flex-flow: row nowrap;
    justify-content: center;
    overflow: clip;
  }
</style>
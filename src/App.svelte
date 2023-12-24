<script lang="ts">
  import ConnectionBar from "./lib/ConnectionBar.svelte";
  import {onDestroy, onMount} from "svelte";
  import {listen} from "@tauri-apps/api/event";
  import PatternBuilder from "./lib/PatternBuilder.svelte";
  import type {PatternBuilderView} from "./lib/pattern_builder/pattern-builder-view.js";

  let connection: {ip: string} | null = null;
  let unlistenOpen, unlistenClose;

  let patternBuilder: PatternBuilderView|null = null;

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
    <PatternBuilder bind:patternBuilder={patternBuilder} />
  </main>
  <ConnectionBar bind:connection={connection} bind:patternBuilder={patternBuilder} />
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
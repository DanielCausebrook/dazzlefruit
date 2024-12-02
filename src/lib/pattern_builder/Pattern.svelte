<script lang="ts">
    import Layer from "./Layer.svelte";
    import Property from "./Property.svelte";
    import {PatternInfo} from "./pattern-builder-view";
    import {listen, type UnlistenFn, type Event as TaruiEvent} from "@tauri-apps/api/event";
    import {onDestroy, onMount} from "svelte";
    import {invoke} from "@tauri-apps/api/core";
    import {PatternView} from "./pattern-view";
    import { createEventDispatcher } from 'svelte';

    const dispatch = createEventDispatcher();

    export let patternInfo: PatternInfo;
    let pattern: PatternView|null = null;
    let unlistenPixelUpdate: UnlistenFn;

    onMount(async () => {
        let patternViewData = JSON.parse(await invoke("view_pattern", {id: patternInfo.id}));
        pattern = new PatternView(patternInfo, patternViewData);
        unlistenPixelUpdate = await listen('pixel-update', async (event: TaruiEvent<{id: RandId, pixel_data: [[number]]}>) => {
            if (event.payload.id === pattern.info.id) {
                let colors = [];
                for (const pixel of event.payload.pixel_data) {
                    colors.push(`rgba(${pixel[0]}, ${pixel[1]}, ${pixel[2]}, ${pixel[3]/2.55}%)`);
                }
                pattern.preview_colors = colors;
                dispatch("pixel-update", { pixelData: event.payload.pixel_data });
            }
        });
    });

    onDestroy(async () => {
        unlistenPixelUpdate();
    });
</script>
{#if pattern !== null}
    <div id="pattern-{pattern.info.id}" class="df-pattern">
        <div class="df-pixel-preview">
            {#key pattern.preview_colors}
                {#each pattern.preview_colors as pixel}
                    <span style="background: {pixel};"></span>
                {/each}
            {/key}
        </div>
        <div class="main">
            <div class="tree">
                <div class="header">Structure</div>
                <div class="main">
                    <Property bind:pattern={pattern} propConfig={pattern.getRootStack()} />
                </div>
            </div>
            {#if pattern.selectedLayerId ?? null !== null}
                <div class="config">
                    <div class="header">Layer Configuration</div>
                    <div class="main">
                        {#key pattern.selectedLayerId}
                            <Layer bind:pattern={pattern} layerId={pattern.selectedLayerId} paneType="Config" />
                        {/key}
                    </div>
                </div>
            {/if}
        </div>
    </div>
{/if}
<style lang="scss">
  @use "theme" as *;

  .df-pattern {
    flex: 1 1 auto;
    display: flex;
    flex-flow: column nowrap;
    overflow: clip;
    > .main {
      flex: 1 1 auto;
      display: flex;
      flex-flow: row nowrap;
      text-align: left;
      overflow: clip;
      background: $bg-m2;
      gap: 2px;
      > * {
        display: flex;
        flex-flow: column nowrap;
        overflow: clip;
        //border: 2px solid $bg-3;
        background: $bg;
        &.tree {
          flex: 0 1 300px;
        }
        &.config {
          flex: 2 2 auto;
        }

        > .header {
          flex: 0 0 auto;
          padding: 5px 10px;
          background: hsl(0, 0%, 13%);
          color: hsl(0, 0%, 70%);
          font-weight: bold;
        }
        > .main {
          flex: 1 1 auto;
          padding: 5px;
          overflow: auto;
        }
      }
    }
    > .df-pixel-preview {
      flex: 0 0 auto;
    }
  }
</style>
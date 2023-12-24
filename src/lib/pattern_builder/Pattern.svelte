<script lang="ts">
    import Layer from "./Layer.svelte";
    import Property from "./Property.svelte";
    import {PatternInfo} from "./pattern-builder-view";
    import {listen, type UnlistenFn} from "@tauri-apps/api/event";
    import {onDestroy, onMount} from "svelte";
    import {invoke} from "@tauri-apps/api/tauri";
    import {PatternView} from "./pattern-view";

    export let patternInfo: PatternInfo;
    let pattern: PatternView|null = null;
    let unlistenPixelUpdate: UnlistenFn;

    onMount(async () => {
        let patternViewData = JSON.parse(await invoke("view_pattern", {id: patternInfo.id}));
        pattern = new PatternView(patternInfo, patternViewData);
        unlistenPixelUpdate = await listen('pixel-update', async (event: Event<{id: RandId, pixel_data: [[number]]}>) => {
            if (event.payload.id === pattern.info.id) {
                let colors = [];
                for (const pixel of event.payload.pixel_data) {
                    colors.push(`rgba(${pixel[0]}, ${pixel[1]}, ${pixel[2]}, ${pixel[3]/2.55}%)`);
                }
                pattern.preview_colors = colors
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
      > * {
        overflow: auto;
        border: 2px solid hsl(0, 0%, 10%);
        &.tree {
          flex: 0 1 300px;
        }
        &.config {
          flex: 2 2 auto;
        }

        > .header {
          padding: 5px 10px;
          background: hsl(0, 0%, 13%);
          color: hsl(0, 0%, 70%);
          font-weight: bold;
        }
        > .main {
          padding: 5px;
        }
      }
    }
    > .df-pixel-preview {
      flex: 0 0 auto;
    }
  }
</style>
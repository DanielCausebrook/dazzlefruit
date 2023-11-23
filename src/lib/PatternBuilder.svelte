<script lang="ts">
    import {invoke} from "@tauri-apps/api/tauri"
    import {onDestroy, onMount} from "svelte";
    import {listen} from "@tauri-apps/api/event";
    import Layer from "./pattern_builder/Layer.svelte";
    import {rgbToHex} from "./pattern_builder/rgb-to-hex";
    import {PatternBuilder} from "./pattern_builder/pattern-builder";

    let id = "something";
    let unlistenPixelUpdate;
    let pixelColors: [string] = [];
    let patternBuilderData: PatternBuilder|null = null;


    onMount(async () => {
        invoke("get_pattern_config", {})
            .then((pattern_builder_str: string) => {
                console.log("OK");
                patternBuilderData = new PatternBuilder(JSON.parse(pattern_builder_str));
            });
        unlistenPixelUpdate = await listen('pixel-update', (event: Event<{id: number, pixel_data: [[number]]}>) => {
            let colors = [];
            for (const pixel of event.payload.pixel_data) {
                colors.push(`rgba(${pixel[0]}, ${pixel[1]}, ${pixel[2]}, ${pixel[3]/2.55}%)`);
            }
            pixelColors = colors;
        });
    });

    onDestroy(async () => {
        unlistenPixelUpdate();
    });
</script>
<div id="pattern-builder-{id}" class="df-pattern-builder">
    <h2>Pattern Builder</h2>
    <div class="df-pixel-preview">
        {#each pixelColors as pixel}
            <span style="background: {pixel};"></span>
        {/each}
    </div>
    <div class="main">
        <div class="tree">
            <div class="header">Structure</div>
            <div class="main">
                {#if patternBuilderData !== null}
                    <Layer bind:patternBuilderData={patternBuilderData} layerId={patternBuilderData.getRootId()} paneType="Tree" />
                {/if}
            </div>
        </div>
        {#if patternBuilderData?.selectedId ?? null !== null}
            <div class="config">
                <div class="header">Layer Configuration</div>
                <div class="main">
                    {#key patternBuilderData.selectedId}
                        <Layer bind:patternBuilderData={patternBuilderData} layerId={patternBuilderData.selectedId} paneType="Config" />
                    {/key}
                </div>
            </div>
        {/if}
    </div>
</div>
<style lang="scss">
    h2 {
      text-align: center;
    }
    .df-pattern-builder {
      flex: 1 1 auto;
      display: flex;
      flex-flow: column nowrap;
      max-width: 1000px;
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
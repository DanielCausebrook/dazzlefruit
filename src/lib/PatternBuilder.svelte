<script lang="ts">
    import {invoke} from "@tauri-apps/api/tauri"
    import {onMount} from "svelte";
    import {PatternBuilderView} from "./pattern_builder/pattern-builder-view";
    import Pattern from "./pattern_builder/Pattern.svelte";
    import { open, message } from "@tauri-apps/api/dialog";
    import Preview3D from "./pattern_builder/Preview3D.svelte";

    export type PositionMap = [[number, number, number]|null];

    export let patternBuilder: PatternBuilderView|null;

    let root: HTMLElement;
    let preview3d: Preview3D;
    let pixelColorData: [[number, number, number, number]] = [];

    onMount(async () => {
        let openPatternsInfo = JSON.parse(await invoke("view_open_patterns", {}));
        patternBuilder = new PatternBuilderView(openPatternsInfo);
        patternBuilder.selectedPatternId = patternBuilder.getPatternsInfo()[0].id;

        let positionMap: PositionMap = JSON.parse(await invoke("position_map", {}))
        preview3d.updatePositionMap(positionMap);
    });

    function updatePixelPreview(event: CustomEvent<{pixelData: [[number, number, number, number]]}>) {
        pixelColorData = event.detail.pixelData;
    }

    async function loadPositionMap() {
        let button = root.querySelector(".load-position-map-button");
        button.disabled = true;
        let path = await open({
            filters: [{
                name: 'JSON File',
                extensions: ['json']
            }]
        });
        try {
            let positionMap: PositionMap = JSON.parse(await invoke("load_position_map", {path: path}));
            preview3d.updatePositionMap(positionMap);
            button.disabled = false;
        } catch (e) {
            await message("Invalid Position Map file.");
        }
    }
</script>
{#if patternBuilder !== null}
    <div class="df-pattern-builder" bind:this={root}>
        <div class="header">
            <h3>Position Map</h3>
        </div>
        <div class="config">
            <div class="preview-3d">
                <Preview3D bind:this={preview3d} bind:pixelColorData={pixelColorData} />
            </div>
            <button class="load-position-map-button" type="button" on:click={loadPositionMap}>Load Position Map...</button>
        </div>
        <div class="pattern-tabs">
            {#each patternBuilder?.getPatternsInfo() ?? [] as patternInfo}
                <div
                        class="tab {patternBuilder.selectedPatternId === patternInfo.id ? 'active' : ''}"
                        on:click={() => { patternBuilder.selectedPatternId = patternInfo.id }}
                >{patternInfo.name}</div>
            {/each}
        </div>
        <div class="pattern-view">
            {#key patternBuilder.selectedPatternId}
                {#if patternBuilder.selectedPatternId !== null }
                    <Pattern patternInfo="{patternBuilder.getPatternInfo(patternBuilder.selectedPatternId)}" on:pixel-update={updatePixelPreview} />
                {/if}
            {/key}
        </div>
    </div>
{/if}
<style lang="scss">
  @use 'theme' as *;

  .df-pattern-builder {
    flex: 1 1 auto;
    display: grid;
    grid:
        "header pattern-tabs" 40px
        "config pattern-view" 1fr
        / auto 1fr;
    align-items: stretch;
    justify-items: stretch;
    overflow: clip;

    > .header {
      grid-area: header;
      display: flex;
      flex-flow: row nowrap;
      align-items: center;
      background: $bg-m1;
      border-right: 1px solid $bg-3;
      color: $fg-2;
      padding: 10px 20px;
    }

    > .config {
      grid-area: config;
      display: flex;
      flex-flow: column nowrap;
      gap: 10px;
      padding: 10px;
      background: $bg-m1;
      border-right: 1px solid $bg-3;
      min-width: max(20vw, 300px);

      &.collapsed {

      }

      > .preview-3d {
        height: 200px;
      }
    }


    > .pattern-tabs {
      grid-area: pattern-tabs;
      display: flex;
      flex-flow: row nowrap;
      gap: 2px;
      padding: 3px 10px 0 10px;
      border-bottom: 1px solid $bg-4;
      overflow: clip;

      > .tab {
        display: flex;
        flex-flow: row nowrap;
        align-items: center;
        justify-items: center;
        padding: 3px 15px;
        cursor: pointer;
        color: $fg-2;
        white-space: nowrap;

        &:hover {
          background: $bg-1;
          color: $fg-m2;
        }

        &.active {
          border-bottom: 3px solid $accent;
          margin-bottom: -1px;
          padding-bottom: 1px;
          color: $fg-m2;
        }
      }
    }

    > .pattern-view {
      grid-area: pattern-view;
      display: flex;
      flex-flow: row nowrap;
      justify-content: center;
      overflow: clip;
    }
  }
</style>
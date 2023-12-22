<script lang="ts">
    import {invoke} from "@tauri-apps/api/tauri"
    import {onMount} from "svelte";
    import {PatternBuilderView} from "./pattern_builder/pattern-builder-view";
    import Pattern from "./pattern_builder/Pattern.svelte";

    export let patternBuilder: PatternBuilderView|null;

    let patternView = null;
    $: patternBuilder?.getPatternView(patternBuilder.selectedPatternId)
        .then(result => {
            patternView = result;
        });

    onMount(async () => {
        let openPatternsInfo = JSON.parse(await invoke("view_open_patterns", {}));
        patternBuilder = new PatternBuilderView(openPatternsInfo);
        patternBuilder.selectedPatternId = patternBuilder.getPatternsInfo()[0].id;
    });
</script>
<div class="df-pattern-builder">
    <div class="pattern-tabs">
        {#each patternBuilder?.getPatternsInfo() ?? [] as patternInfo}
            <div
                    class="tab {patternBuilder.selectedPatternId === patternInfo.id ? 'active' : ''}"
                    on:click={() => { patternBuilder.selectedPatternId = patternInfo.id }}
            >{patternInfo.name}</div>
        {/each}
    </div>
    <div class="pattern-view">
        {#key patternView}
            {#if patternView !== null }
                <Pattern pattern="{patternView}" />
            {/if}
        {/key}
    </div>
</div>
<style lang="scss">
    .df-pattern-builder {
      flex: 1 1 auto;
      display: grid;
      grid:
        "pattern-tabs" 40px
        "pattern-view" 1fr
        / 1fr;
      align-items: stretch;
      justify-items: stretch;

      > .pattern-tabs {
        grid-area: pattern-tabs;
        display: flex;
        flex-flow: row nowrap;
        gap: 2px;
        padding: 3px 10px 0 10px;
        border-bottom: 1px solid hsl(0, 0%, 30%);

        > .tab {
          display: flex;
          flex-flow: row nowrap;
          align-items: center;
          justify-items: center;
          padding: 3px 15px;
          cursor: pointer;
          color: hsl(0, 0%, 75%);

          &:hover {
            background: hsla(0, 0%, 0%, 20%);
            color: inherit;
          }

          &.active {
            border-bottom: 3px solid hsl(240, 80%, 70%);
            margin-bottom: -1px;
            padding-bottom: 1px;
            color: inherit;
          }
        }
      }
      > .pattern-view {
        grid-area: pattern-view;
        display: flex;
        flex-flow: row nowrap;
        justify-content: center;
      }
    }
</style>
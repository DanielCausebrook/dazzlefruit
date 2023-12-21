<script lang="ts">
    import Property from "./Property.svelte";
    import {
        IconAdjustments,
        IconFolderOpen,
        IconTexture,
        IconHexagonalPrism,
        IconFilter,
        IconQuestionMark,
        IconArrowBigRight
    } from "@tabler/icons-svelte";
    import type {PatternBuilder} from "./pattern-builder";

    export let patternBuilderData: PatternBuilder;
    export let layerId: RandId;

    let layerConfig: Component = patternBuilderData.getLayerConfig(layerId);
    export let paneType: "Tree"|"Config";
</script>
<div class="df-layer {patternBuilderData.selectedId === layerId && paneType === 'Tree'? 'selected' : ''}">
    <div class="header" on:click={() => {
        patternBuilderData.setSelectedLayer(layerId);
        patternBuilderData = patternBuilderData;
    }}>
        <div class="layer-icon">
            {#if layerConfig.type === 'Generic'}
                <IconQuestionMark stroke={patternBuilderData.selectedId === layerId ? 2 : 1}/>
            {:else if layerConfig.type === 'Texture'}
                <IconTexture stroke={patternBuilderData.selectedId === layerId ? 2 : 1}/>
            {:else if layerConfig.type === 'Filter'}
                <IconFilter stroke={patternBuilderData.selectedId === layerId ? 2 : 1}/>
            {:else if layerConfig.type === 'Group'}
                <IconFolderOpen stroke={patternBuilderData.selectedId === layerId ? 2 : 1}/>
            {:else if layerConfig.type === 'Transformer'}
                <IconArrowBigRight stroke={patternBuilderData.selectedId === layerId ? 2 : 1}/>
            {:else if layerConfig.type === 'texture-generator'}
                <IconHexagonalPrism stroke={patternBuilderData.selectedId === layerId ? 2 : 1}/>
            {/if}
        </div>
        <span>{layerConfig.name.value}</span>
        <div class="config-icon">
            <IconAdjustments size=20 stroke=1 color="hsl(0, 0%, 80%)"/>
        </div>
    </div>
    <div class="properties">
        {#each layerConfig.properties as property}
            {#if property.display_pane === 'TreeAndConfig' || (paneType === 'Tree' && property.display_pane === 'Tree') || (paneType === 'Config' && property.display_pane === 'Config')}
                <Property bind:patternBuilderData={patternBuilderData} propConfig={property} />
            {/if}
        {/each}
    </div>
</div>
<style lang="scss">
    .df-layer {
      display: flex;
      flex-flow: column nowrap;
      align-items: stretch;

      &.selected {
        background: hsla(220, 40%, 30%, 60%);
        > .header {
          font-weight: bold;
        }
      }

      > .header {
        display: flex;
        flex-flow: row nowrap;
        align-items: center;
        gap: 5px;
        padding: 1px;
        font-size: 90%;
        cursor: pointer;

        &:hover {
          background: hsla(0, 0%, 0%, 38%);
          + .properties {
            background: hsla(0, 0%, 10%, 38%);
          }
        }

        > .layer-icon {
          flex: 0 0 auto;
          display: flex;
        }
        > span {
          flex: 1 1 auto;
        }
        > .config-icon {
          flex: 0 0 auto;
          display: flex;
          padding: 2px;
        }
      }
      > .properties {
        display: flex;
        flex-flow: column nowrap;
        gap: 10px;
        margin: 0;
        padding-left: 13px;
      }
    }
</style>

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
    import type {PatternView} from "./pattern-view";

    export let pattern: PatternView;
    export let layerId: RandId;

    let layerView: Layer = pattern.getLayerView(layerId);
    export let paneType: "Tree"|"Config";
</script>
<div class="df-layer {pattern.selectedLayerId === layerId && paneType === 'Tree'? 'selected' : ''}">
    <div class="header" on:click={() => {
        pattern.selectedLayerId = layerId;
    }}>
        <div class="layer-icon">
            {#if layerView.type.icon === null}
                <IconQuestionMark stroke={pattern.selectedLayerId === layerId ? 2 : 1}/>
            {:else if layerView.type.icon === 'Texture'}
                <IconTexture stroke={pattern.selectedLayerId === layerId ? 2 : 1}/>
            {:else if layerView.type.icon === 'Filter'}
                <IconFilter stroke={pattern.selectedLayerId === layerId ? 2 : 1}/>
            {:else if layerView.type.icon === 'Group'}
                <IconFolderOpen stroke={pattern.selectedLayerId === layerId ? 2 : 1}/>
            {:else if layerView.type.icon === 'Transformer'}
                <IconArrowBigRight stroke={pattern.selectedLayerId === layerId ? 2 : 1}/>
            <!--{:else if layerView.type.icon === 'texture-generator'}-->
            <!--    <IconHexagonalPrism stroke={pattern.selectedLayerId === layerId ? 2 : 1}/>-->
            {/if}
        </div>
        {#if layerView.name.value === null}
            <span class="layer-type-name">{layerView.type.name}</span>
        {:else}
            <span class="layer-name">{layerView.name.value}</span>
        {/if}
        <div class="config-icon">
            <IconAdjustments size=20 stroke=1 color="hsl(0, 0%, 80%)"/>
        </div>
    </div>
    <div class="properties">
        {#each layerView.properties as property}
            {#if property.display_pane === 'TreeAndConfig' || (paneType === 'Tree' && property.display_pane === 'Tree') || (paneType === 'Config' && property.display_pane === 'Config')}
                <Property bind:pattern={pattern} bind:propConfig={property} />
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
          > .layer-name {
            font-weight: 700;
          }
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
        > .layer-type-name {
          color: hsl(0, 0%, 80%);
        }
        > .layer-name {
          font-weight: 500;
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

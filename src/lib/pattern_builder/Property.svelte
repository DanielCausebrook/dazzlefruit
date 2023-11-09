<script lang="ts">
    import Layer from "./Layer.svelte";
    import {invoke} from "@tauri-apps/api/tauri";
    import {rgbToHex} from "./rgb-to-hex";
    import type {PatternBuilder} from "./pattern-builder";

    export let patternBuilderData: PatternBuilder;
    export let propConfig: Property;

    let color: string|null;
    if (propConfig.property_type === "color") {
        $: color = rgbToHex(propConfig.value[0], propConfig.value[1], propConfig.value[2])
    }

    async function updateNum() {
        await invoke("update_property", {id:propConfig.id, value:propConfig.value.toString()})
            .then(() => {
                console.log("OK");
                // message = "Connection Success!";
            })
            .catch((reason) => {
                console.log(reason);
            });
    }

    async function updateColor() {
        await invoke("update_property", {id:propConfig.id, value:color})
            .then(() => {
                console.log("OK");
                // message = "Connection Success!";
            })
            .catch((reason) => {
                console.log(reason);
            });
    }
</script>
<div class="df-property">
    {#if propConfig.name !== null}
        <div class="header">{propConfig.name}</div>
    {/if}
    {#if propConfig.property_type === "layerVec" || propConfig.property_type === "pixelBlueprintVec"}
        <div class="value layer-vec">
            {#each propConfig.value as layerId}
                <Layer bind:patternBuilderData={patternBuilderData} layerId={layerId} paneType="Tree" />
            {/each}
        </div>
    {:else if propConfig.property_type === "pixelLayer" || propConfig.property_type === "textureProducer"}
        <div class="value layer">
            <Layer bind:patternBuilderData={patternBuilderData} layerId={propConfig.value} paneType="Tree" />
        </div>
    {:else if propConfig.property_type === "num"}
        <div class="value input">
            <input
                    type="range"
                    step="{propConfig.slider.step}"
                    min="{propConfig.slider.range.start}"
                    max="{propConfig.slider.range.end}"
                    bind:value={propConfig.value}
                    on:input={updateNum}
            />
            <input
                    type="number"
                    step="{propConfig.slider.step}"
                    min="{propConfig.slider.range.start}"
                    max="{propConfig.slider.range.end}"
                    bind:value={propConfig.value}
                    on:change={updateNum}
            />
        </div>
    {:else if propConfig.property_type === "color"}
        <div class="value input">
            <input
                    type="color"
                    bind:value={color}
                    on:change={updateColor}
            />
        </div>
    {/if}
</div>
<style lang="scss">
  .df-property {
    border-left: 1px solid hsl(0, 0%, 8%);

    > .header {
      padding: 2px 3px 1px;
      font-size: 80%;
      color: hsl(0, 0%, 80%)
    }

    > .value {
      margin-left: 5px;
      padding-left: 8px;

      &.input {
        display: flex;
        flex-flow: row nowrap;
        align-items: center;
        gap: 5px;
      }

      &.layer-vec {
        display: flex;
        flex-flow: column nowrap;
        //border-left: 1px solid hsl(0, 0%, 8%);
        padding-left: 0;
        gap: 5px;
      }

      &.layer {
        //border-left: 1px solid hsl(0, 0%, 8%);
        padding-left: 0;
      }

      > input {
        min-width: 80px;
        padding: 2px 8px;
        border-radius: 3px;
      }
    }
  }
</style>
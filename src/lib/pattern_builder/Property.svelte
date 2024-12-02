<script lang="ts">
    import Layer from "./Layer.svelte";
    import {invoke} from "@tauri-apps/api/core";
    import {rgbToHex} from "./rgb-to-hex";
    import type {PatternView} from "./pattern-view";

    export let pattern: PatternView;
    export let propConfig: AnyPropView;

    let color: string|null;
    let outputError = null;
    let errorMap = new Map();
    if (propConfig.type === "color") {
        color = rgbToHex(propConfig.value[0], propConfig.value[1], propConfig.value[2])
    } else if (propConfig.type === "layer-stack") {
        for (const error of propConfig.data.errors) {
            if (error.layer_id !== null) {
                errorMap.set(error.layer_id, error);
            } else {
                outputError = error;
            }
        }
    }
    const numVecLabels = ['x', 'y', 'z'];

    async function updateWith(valueStr) {
        try {
            await invoke("update_property", {patternId:pattern.info.id, propId:propConfig.id, value:valueStr});
        } catch (err) {
            console.log(err);
        }
    }

    async function updateStringify() {
        await updateWith(JSON.stringify(propConfig.value));
    }

    async function updateColor() {
        await updateWith(color);
    }
</script>
{#if propConfig.type !== "raw"}
    <div class="df-property">
        {#if propConfig.name !== null}
            <div class="header">{propConfig.name}</div>
        {/if}
        {#if propConfig.type === "layer-vec" }
            <div class="value layer-vec">
                {#each propConfig.value as layerId}
                    <Layer bind:pattern={pattern} layerId={layerId} paneType="{propConfig.display_pane}" />
                {/each}
            </div>
        {:else if propConfig.type === "layer-stack" }
            <div class="value layer-stack">
                {#each propConfig.value as layerId}
                    {@const error = errorMap.get(layerId) ?? null}
                    {#if error !== null }
                        <div class="layer-stack-type-error">Cannot convert {error.from_type_name} into {error.into_type_name}</div>
                    {/if}
                    <Layer bind:pattern={pattern} layerId={layerId} paneType="{propConfig.display_pane}" />
                {/each}
                {#if outputError !== null }
                    <div class="layer-stack-type-error">Cannot convert {outputError.from_type_name} into {outputError.into_type_name}</div>
                {/if}
            </div>
        {:else if propConfig.type === "layer" }
            <div class="value layer">
                <Layer bind:pattern={pattern} layerId={propConfig.value} paneType="{propConfig.display_pane}" />
            </div>
        {:else if propConfig.type === "num"}
            <div class="value input">
                {#if propConfig.data.slider !== null}
                    <input
                            type="range"
                            step="{propConfig.data.slider.step}"
                            min="{propConfig.data.slider.range.start}"
                            max="{propConfig.data.slider.range.end}"
                            bind:value={propConfig.value}
                            on:input={updateStringify}
                    />
                    <input
                            type="number"
                            step="{propConfig.data.slider.step}"
                            min="{propConfig.data.slider.range.start}"
                            max="{propConfig.data.slider.range.end}"
                            bind:value={propConfig.value}
                            on:change={updateStringify}
                    />
                {:else}
                    <input
                            type="number"
                            bind:value={propConfig.value}
                            on:change={updateStringify}
                    />
                {/if}
            </div>
        {:else if propConfig.type === "num-vec"}
            {#each propConfig.value as _, i}
                <div class="value input">
                    {#if numVecLabels.length > i}
                        <label for="{propConfig.id}" class="label">{numVecLabels[i]}</label>
                    {/if}
                    {#if propConfig.data.sliders[i] !== null}
                        <input
                                type="range"
                                step="{propConfig.data.sliders[i].step}"
                                min="{propConfig.data.sliders[i].range.start}"
                                max="{propConfig.data.sliders[i].range.end}"
                                bind:value={propConfig.value[i]}
                                on:input={updateStringify}
                        />
                        <input
                                id="{propConfig.id}"
                                type="number"
                                step="{propConfig.data.sliders[i].step}"
                                min="{propConfig.data.sliders[i].range.start}"
                                max="{propConfig.data.sliders[i].range.end}"
                                bind:value={propConfig.value[i]}
                                on:change={updateStringify}
                        />
                    {:else}
                        <input
                                id="{propConfig.id}"
                                type="number"
                                bind:value={propConfig.value[i]}
                                on:change={updateStringify}
                        />
                    {/if}
                </div>
            {/each}
        {:else if propConfig.type === "string" }
            <div class="value input">
                <input
                        type="text"
                        bind:value={propConfig.value}
                        on:change={updateStringify}
                />
            </div>
        {:else if propConfig.type === "color"}
            <div class="value input">
                <input
                        type="color"
                        bind:value={color}
                        on:change={updateColor}
                />
            </div>
        {/if}
    </div>
{/if}
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
        > .label {
          color: hsl(0, 0%, 65%);
        }
      }

      &.layer-vec, &.layer-stack {
        display: flex;
        flex-flow: column nowrap;
        //border-left: 1px solid hsl(0, 0%, 8%);
        padding-left: 0;
        gap: 5px;

        > .layer-stack-type-error {
          padding: 1px 5px;
          font-size: 80%;
          background: hsla(0, 80%, 50%, 30%);
          color: hsl(0, 80%, 80%);
        }
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
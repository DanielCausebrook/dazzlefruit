<script lang="ts">
    import Layer from "./Layer.svelte";
    import {invoke} from "@tauri-apps/api/tauri";
    import {rgbToHex} from "./rgb-to-hex";
    import type {PatternBuilder} from "./pattern-builder";

    export let patternBuilderData: PatternBuilder;
    export let propConfig: AnyPropView;

    let color: string|null;
    if (propConfig.type === "color") {
        $: color = rgbToHex(propConfig.value[0], propConfig.value[1], propConfig.value[2])
    }

    async function update() {
        await invoke("update_property", {id:propConfig.id, value:JSON.stringify(propConfig.value)})
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
{#if propConfig.type !== "raw"}
    <div class="df-property">
        {#if propConfig.name !== null}
            <div class="header">{propConfig.name}</div>
        {/if}
        {#if propConfig.type === "component-vec" }
            <div class="value layer-vec">
                {#each propConfig.value as layerId}
                    <Layer bind:patternBuilderData={patternBuilderData} layerId={layerId} paneType="{propConfig.display_pane}" />
                {/each}
            </div>
        {:else if propConfig.type === "component" }
            <div class="value layer">
                <Layer bind:patternBuilderData={patternBuilderData} layerId={propConfig.value} paneType="{propConfig.display_pane}" />
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
                            on:input={update}
                    />
                    <input
                            type="number"
                            step="{propConfig.data.slider.step}"
                            min="{propConfig.data.slider.range.start}"
                            max="{propConfig.data.slider.range.end}"
                            bind:value={propConfig.value}
                            on:change={update}
                    />
                {:else}
                    <input
                            type="number"
                            bind:value={propConfig.value}
                            on:change={update}
                    />
                {/if}
            </div>
        {:else if propConfig.type === "num-vec"}
            {#each propConfig.value as _, i}
                <div class="value input">
                    {#if propConfig.data.sliders[i] !== null}
                        <input
                                type="range"
                                step="{propConfig.data.sliders[i].step}"
                                min="{propConfig.data.sliders[i].range.start}"
                                max="{propConfig.data.sliders[i].range.end}"
                                bind:value={propConfig.value[i]}
                                on:input={update}
                        />
                        <input
                                type="number"
                                step="{propConfig.data.sliders[i].step}"
                                min="{propConfig.data.sliders[i].range.start}"
                                max="{propConfig.data.sliders[i].range.end}"
                                bind:value={propConfig.value[i]}
                                on:change={update}
                        />
                    {:else}
                        <input
                                type="number"
                                bind:value={propConfig.value[i]}
                                on:change={update}
                        />
                    {/if}
                </div>
            {/each}
        {:else if propConfig.type === "string" }
            <div class="value input">
                <input
                        type="text"
                        bind:value={propConfig.value}
                        on:change={update}
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
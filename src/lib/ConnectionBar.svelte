<script lang="ts">
    import {invoke} from "@tauri-apps/api/core"
    import { emit, listen } from '@tauri-apps/api/event';
    import { onMount, onDestroy } from 'svelte';
    import type {PatternBuilderView} from "./pattern_builder/pattern-builder-view";
    export let connection: {ip: string} | null = null;
    export let patternBuilder: PatternBuilderView|null;

    let selectedPatternId: string = "";

    async function disconnect() {
        try {
            await invoke("disconnect", {});
            selectedPatternId = "";
        } catch (err) {
            // message = err;
            // setTimeout(() => message = "", 5000);
        }
    }

    let ip = "192.168.1.135";
    let tcpPort = 4242;
    let udpPort = 4243;
    let numPixels = 100;

    let message = "";

    async function connect() {
        message = "";
        try {
            await invoke("connect", {ip:ip, tcpPort:Number(tcpPort), udpPort:Number(udpPort)});
        } catch (err) {
            message = err;
            setTimeout(() => message = "", 5000);
            return;
        }
        try {
            await invoke("init_neopixel", {numPixels:Number(numPixels)});
        } catch (err) {
            // message = err;
            // setTimeout(() => message = "", 5000);
        }
    }

    async function setPattern() {
        try {
            await invoke("set_neopixel_pattern", {patternId: selectedPatternId !== "" ? selectedPatternId : null});
        } catch (err) {
            message = err;
            setTimeout(() => message = "", 5000);
        }
    }
</script>

<div class="df-connection-bar">
    {#if connection === null}
        <form class="connect-form" on:submit|preventDefault={connect}>
            <label for="df-connection-bar-connect-ip">IP:</label>
            <input id="df-connection-bar-connect-ip" size="15" class="input-small" bind:value={ip} />&nbsp;&nbsp;
            <label for="df-connection-bar-connect-num-pixels">Pixels:</label>
            <input type="number" step="50" min="0" id="df-connection-bar-connect-num-pixels" class="input-small" bind:value={numPixels} />&nbsp;&nbsp;
            <button type="submit" class="button-small">Connect</button>
        </form>
        <div class="spacer"></div>
        <div>No Pico Connected</div>
    {:else}
        {#if patternBuilder !== null}
            <div class="pattern-select">
                <label for="df-connection-bar-pattern-select">Live Pattern:</label>
                <select id="df-connection-bar-pattern-select" bind:value={selectedPatternId} on:change={setPattern}>
                    <option value="" selected>Off</option>
                    {#each patternBuilder.getPatternsInfo() as patternInfo}
                        <option value="{patternInfo.id}">{patternInfo.name}</option>
                    {/each}
                </select>
            </div>
        {/if}
        <div class="spacer"></div>
        <div class="connected">Connected</div>
        <div class="connected">{connection.ip}</div>
        <div class="disconnect" on:click={disconnect}>&#215;</div>
    {/if}
    <span class="message">{message}</span>
</div>

<style lang="scss">
  .df-connection-bar {
    flex: 0 0 auto;
    display: flex;
    position: relative;
    flex-flow: row nowrap;
    align-items: center;
    justify-content: flex-end;
    gap: 10px;
    height: 25px;
    padding: 5px 15px;
    background: hsl(0, 0%, 10%);
    color: hsl(0, 0%, 50%);

    .disconnect {
      background: hsl(0, 0%, 18%);
      color: hsl(0, 0%, 40%);
      padding: 0 3px;
      line-height: 100%;
      cursor: pointer;

      &:hover {
        background: hsl(0, 0%, 25%);
        color: hsl(0, 50%, 50%);
      }
    }
    .connected {
      color: hsl(120, 25%, 45%);
    }
    .connect-form {
      justify-self: flex-start;
      font-size: 0.8em;
      margin: 0 10px;
    }
    .spacer {
      flex: 1 0 auto;
    }
    .message {
      position: absolute;
      left: 50%;
      top: -5px;
      transform: translate(-50%, -100%);
      padding: 5px 10px;
      border-radius: 5px;
      background: hsla(0, 0%, 0%, 80%);

      &:empty {
        display: none;
      }
    }
  }
  #df-connection-bar-connect-num-pixels {
    width: 50px;
  }
</style>

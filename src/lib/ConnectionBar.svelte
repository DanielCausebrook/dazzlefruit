<script lang="ts">
    import {invoke} from "@tauri-apps/api/tauri"
    import { emit, listen } from '@tauri-apps/api/event';
    import { onMount, onDestroy } from 'svelte';
    export let connection: {ip: string} | null = null;

    async function disconnect() {
        await invoke("disconnect", {})
            .then(() => {})
            .catch((reason) => {});
    }

    let ip = "192.168.1.135";
    let tcpPort = 4242;
    let udpPort = 4243;

    let message = "";

    async function connect() {
        message = "";
        await invoke("connect", {ip:ip, tcpPort:Number(tcpPort), udpPort:Number(udpPort)})
            .then(() => {
                // message = "Connection Success!";
            })
            .catch((reason) => {
                message = reason;
                setTimeout(() => message = "", 5000);
            });
        await invoke("init_neopixel", {numPixels: 100})
            .then(() => {
            })
            .catch((reason) => {
                // message = reason;
                // setTimeout(() => message = "", 5000);
            });
    }
</script>

<style>
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
    }
    .df-connection-bar .disconnect {
        background: hsl(0, 0%, 18%);
        color: hsl(0, 0%, 40%);
        padding: 0 3px;
        line-height: 100%;
        cursor: pointer;
    }
    .df-connection-bar .connected {
        color: hsl(120, 25%, 45%);
    }
    .df-connection-bar .disconnect:hover {
         background: hsl(0, 0%, 25%);
         color: hsl(0, 50%, 50%);
    }
    .df-connection-bar .connect-form {
        justify-self: flex-start;
        font-size: 0.8em;
        margin: 0 10px;
    }
    .df-connection-bar .spacer {
        flex: 1 0 auto;
    }
    .df-connection-bar .message {
        position: absolute;
        left: 50%;
        top: -5px;
        transform: translate(-50%, -100%);
        padding: 5px 10px;
        border-radius: 5px;
        background: hsla(0, 0%, 0%, 80%);
    }
    .df-connection-bar .message:empty {
        display: none;
    }
</style>

<div class="df-connection-bar">
    {#if connection === null}
        <form class="connect-form" on:submit|preventDefault={connect}>
            <label for="connect-ip">IP:</label>
            <input id="connect-ip" size="15" class="input-small" bind:value={ip} />&nbsp;&nbsp;
            <button type="submit" class="button-small">Connect</button>
        </form>
        <div class="spacer"></div>
        <div>No Pico Connected</div>
    {:else}
        <div class="connected">Connected</div>
        <div class="connected">{connection.ip}</div>
        <div class="disconnect" on:click={disconnect}>&#215;</div>
    {/if}
    <span class="message">{message}</span>
</div>

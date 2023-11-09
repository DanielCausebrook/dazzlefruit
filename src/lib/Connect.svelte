<script lang="ts">
    import {invoke} from "@tauri-apps/api/tauri"

    let ip = "192.168.1.135";
    let tcpPort = 4242;
    let udpPort = 4243;

    let message = "";

    async function connect() {
        message = "";
        await invoke("connect", {ip:ip, tcpPort:Number(tcpPort), udpPort:Number(udpPort)})
            .then(() => {
                message = "Connection Success!";
            })
            .catch((reason) => { message = reason; });
    }
</script>

<div>
    <form class="container" on:submit|preventDefault={connect}>
        <h2>Connect to a Pico</h2>
        <div class="row">
            <label for="connect-ip">IP:</label>
            <input id="connect-ip" size="15" bind:value={ip} />&nbsp;&nbsp;
            <label for="connect-tcp-port">TCP Port:</label>
            <input id="connect-tcp-port" type="tel" maxlength="5" size="5" bind:value={tcpPort}>&nbsp;&nbsp;
            <label for="connect-udp-port">UDP Port:</label>
            <input id="connect-udp-port" type="tel" maxlength="5" size="5" bind:value={udpPort}>
        </div>
        <div class="row">
            <button type="submit">Connect</button>
        </div>
    </form>
    <p>{message}</p>
</div>
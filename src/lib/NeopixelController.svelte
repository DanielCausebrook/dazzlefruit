<script>

    import {invoke} from "@tauri-apps/api/tauri";

    let colorStr;
    let message = "";

    async function setColor() {
        await invoke("set_color", {color_str: colorStr})
            .then(() => {

            })
            .catch((reason) => {
                message = reason;
            });
    }
</script>

<div class="container">
    <h2>Neopixel Controller</h2>
    <div class="row">
        <label for="neopixel-color">Color:</label>
        <input id="neopixel-color" type="color" on:change={setColor} bind:value={colorStr}>
    </div>
    <p>{message}</p>
</div>

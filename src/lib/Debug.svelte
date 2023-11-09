<script lang="ts">
    import { emit, listen } from '@tauri-apps/api/event'
    import { onMount, onDestroy } from 'svelte';

    let debugOutput = "";
    let unlisten;

    onMount(async () => {
        unlisten = await listen('debug-println', (event) => {
            debugOutput += event.payload.message + "\n";
            // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
            // event.payload is the payload object
        });
    });

    onDestroy(async () => {
        unlisten();
    })
</script>
<aside class="debug container">
    <header>Debug</header>
    <p>{ #each debugOutput.split("\n") as debugLine }{ debugLine }<br>{ /each }</p>
</aside>
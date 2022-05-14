<script lang="ts">
    import init, { greet } from 'mavinote-wasm';
    import { onMount } from 'svelte';

    onMount(async () => {
        await init();
        console.log(greet());
    });
</script>

<div>
    <h1>Hello World!</h1>
</div>

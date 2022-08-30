<script lang="ts">
    import init from '$lib/wasm';
    import { onMount } from 'svelte';

    import Folders from './folders.svelte';

    let initialized = false;

    onMount(() => init().then(() => (initialized = true)));
</script>

{#if initialized}
    <div>
        <div><Folders /></div>

        <div>
            <slot />
        </div>
    </div>
{/if}

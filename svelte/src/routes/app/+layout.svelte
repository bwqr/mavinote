<script lang="ts">
    import init from '$lib/wasm';

    import Folders from './folders.svelte';

    let initialized = false;

    init().then(() => initialized = true);
</script>

{#if initialized }
<div>
    <div><Folders/></div>

    <div>
        <slot></slot>
    </div>
</div>
{/if }

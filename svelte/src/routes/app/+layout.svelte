<script lang="ts">
    import init from '$lib/wasm';
    import { onMount } from 'svelte';

    import Folders from './folders.svelte';

    let initialized = false;

    onMount(() => init().then(() => (initialized = true)));
</script>

{#if initialized}
    <div class="container-xxl">
        <div class="row">
            <div class="col-md-3">
                <Folders/>
            </div>

            <div class="col overflow-auto" style="height: 100vh;">
                <slot/>
            </div>
        </div>
    </div>
{/if}

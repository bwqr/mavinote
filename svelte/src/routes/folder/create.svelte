<script lang="ts">
    import init from '$lib/wasm';
    import * as noteStore from '$lib/stores/note';
    import { onMount } from 'svelte';
    import { goto } from '$app/navigation';

    let name = '';
    let inProgress = false;

    onMount(async () => await init());

    function create() {
        if (inProgress) {
            return;
        }

        inProgress = true;

        noteStore
            .createFolder(name)
            .then(() => goto('/'))
            .finally(() => (inProgress = false));
    }
</script>

<div>
    <h1>Notes</h1>
    <form on:submit|preventDefault={create}>
        <input type="text" placeholder="Name" bind:value={name} />
        <button type="submit">Create</button>
    </form>
</div>

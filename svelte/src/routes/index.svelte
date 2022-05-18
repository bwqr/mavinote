<script lang="ts">
    import init from '$lib/wasm';
    import type { Folder } from '$lib/models';
    import { onMount } from 'svelte';
    import * as noteStore from '$lib/stores/note';

    let folders: Folder[] = [];

    onMount(async () => {
        await init();
        folders = await noteStore.folders();
    });
</script>

<div>
    <h1>Folders</h1>
    <ul>
        {#each folders as folder}
            <li><a href={`/folder/${folder.id}`}>{folder.name}</a></li>
        {/each}
    </ul>
    <a href="/folder/create">Create</a>
</div>

<script lang="ts">
    import init, { folders as fetchFolders } from 'mavinote-wasm';
    import { onMount } from 'svelte';

    let folders: any[] = [];

    onMount(async () => {
        await init();
        try {
            folders = JSON.parse(await fetchFolders());
        } catch(e) {
            console.error('error in 3, 2, 1 ...', e);
        }
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

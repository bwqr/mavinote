<script lang="ts">
    import type { Folder } from '$lib/models';
    import * as noteStore from '$lib/stores/note';

    let folders: Folder[] = [];
    noteStore.folders().subscribe((f) => (folders = f));

    let folderName = '';
    let error: string | undefined = undefined;
    let inProgress = false;

    function createFolder() {
        if (inProgress) {
            return;
        }

        if (folderName.trim().length === 0) {
            error = 'Please fill the Folder Name';
            return;
        }

        error = undefined;
        inProgress = true;

        noteStore
            .createFolder(folderName)
            .then((f) => {
                folders = [...folders, f];
                folderName = '';
            })
            .finally(() => (inProgress = false));
    }
</script>

<h1>Folders</h1>
<form on:submit|preventDefault={createFolder}>
    <input placeholder="Folder Name" bind:value={folderName} required />
    {#if error}
        <span>{error}</span>
    {/if}
    <button>Create Folder</button>
</form>
<ul>
    {#each folders as folder}
        <li><a href={`/app/folder/${folder.id}`}>{folder.name}</a></li>
    {/each}
</ul>

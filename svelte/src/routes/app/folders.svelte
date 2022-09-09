<script lang="ts">
    import { page } from '$app/stores';
    import type { Folder } from '$lib/models';
    import * as noteStore from '$lib/stores/note';
    import { Subscription } from 'rxjs';
    import { onDestroy } from 'svelte';
    import FolderPlus from '/src/icons/folder-plus.svelte';

    let folders: Folder[] = [];
    let folderName = '';
    let error: string | undefined = undefined;
    let inProgress = false;
    let selectedFolderId: number | undefined = undefined;
    const subs = new Subscription();

    const pageSub = page.subscribe((page) => {
        if (page.routeId?.startsWith('app/folder')) {
            selectedFolderId = parseInt(page.params.id ?? '0');
        }
    })

    subs.add({ unsubscribe: pageSub });
    subs.add(noteStore.folders().subscribe((f) => (folders = f)));
    onDestroy(() => subs.unsubscribe());

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

<div style="height: calc(100vh - 1rem);" class="d-flex flex-column my-2 border-end border-2">
    <div class="overflow-auto flex-grow-1 p-3">
        <h3 class="mb-2">Folders</h3>

        {#if folders.length === 0}
            <small class="text-black-50 d-block mb-2">There is no folder to display.</small>
            <small class="text-black-50 d-block mb-2">In order to start taking notes, you need to create a folder.</small>
            <small class="text-black-50 d-block mb-5">Folders can be created from the dialog displayed in the bottom part.</small>
        {:else}
            <ul class="list-unstyled">
                {#each folders as folder, i}
                    <li class="py-2 border-1" class:border-bottom={i !== folders.length - 1}>
                        <a href={`/app/folder/${folder.id}`} class:selected={selectedFolderId === folder.id} class="d-block p-2 list-element rounded text-decoration-none text-body">
                            {folder.name}
                        </a>
                    </li>
                {/each}
            </ul>
        {/if}
    </div>

    <form on:submit|preventDefault={createFolder} class="mt-2 p-3">
        <h5>Create Folder</h5>
        <div class="mb-2">
            <label class="visually-hidden" for="autoSizingInputGroup">Folder Name</label>
            <div class="input-group">
                <input type="text" class="form-control" id="autoSizingInputGroup" placeholder="Folder Name" bind:value={folderName} required/>
                <button type="submit" class="btn btn-primary"><FolderPlus/></button>
            </div>
        </div>

        {#if error}
            <p class="text-end mb-2 text-danger">{error}</p>
        {/if}
    </form>
</div>

<style lang="scss">
    ul .list-element:hover, ul .selected {
        background-color: var(--bs-gray-200);
    }
</style>

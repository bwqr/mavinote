<script lang="ts">
    import { goto } from '$app/navigation';

    import { page } from '$app/stores';
    import type { Folder, Note } from '$lib/models';
    import * as noteStore from '$lib/stores/note';
    import { Subscription } from 'rxjs';
    import { onDestroy } from 'svelte';
    import PencilSquare from '../../../../icons/pencil-square.svelte';
    import ThreeDotsVertical from '../../../../icons/three-dots-vertical.svelte';

    let folder: Folder | undefined = undefined;
    let folderId: number | undefined = undefined;
    let notes: Note[] = [];
    let deleting = false;
    let subs = new Subscription();
    let showActionMenu = false;

    const sub = page.subscribe(({ params }) => {
        folderId = parseInt(params.id);
        notes = [];

        subs.add(noteStore.notes(folderId).subscribe((n) => (notes = n)));
        subs.add(noteStore.folders().subscribe(folders => folder = folders.find(f => folderId === f.id)));
    });
    subs.add({ unsubscribe: () => sub() });
    onDestroy(() => subs.unsubscribe());

    function deleteFolder() {
        if (deleting) {
            return;
        }

        deleting = true;

        noteStore
            .deleteFolder(parseInt($page.params.id))
            .then(() => goto('/app'))
            .finally(() => (deleting = false));
    }
</script>

<div class="my-2 p-3">
    {#if folder}
        <div class="d-flex border-bottom border-2 pb-2 mb-4">
            <div class="flex-grow-1">
                <h3 class="d-inline-block m-0">{folder.name}</h3>
                <small>Folder</small>
            </div>
            <a class="btn btn-outline-primary me-2" href={`/app/folder/${folderId}/create-note`}><PencilSquare/> Create Note</a>

            <div class="dropdown">
                <button class="btn btn-light" on:click={() => showActionMenu = !showActionMenu}><ThreeDotsVertical/></button>
                <ul class="dropdown-menu position-absolute end-0 mt-1" class:show={showActionMenu}>
                    <li>
                        <button class="dropdown-item text-danger text-center" on:click={() => deleteFolder()}>
                            {#if deleting}
                                <span class="spinner-border spinner-border-sm me-2" role="status" aria-hidden="true"></span>
                            {/if}
                            Delete
                        </button>
                    </li>
                </ul>
            </div>
        </div>
    {/if}

    <h4 class="mb-2">Notes</h4>

    <ul class="list-unstyled">
        {#each notes as note}
            <li class="my-2">
                <a href={`/app/note/${note.id}`} class="d-block p-2 list-element rounded text-decoration-none text-body">
                    {note.title ?? 'New Note'}
                </a>
            </li>
        {/each}
    </ul>
</div>


<style lang="scss">
    ul .list-element:hover, ul .selected {
        background-color: var(--bs-gray-200);
    }
</style>

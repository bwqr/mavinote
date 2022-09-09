<script lang="ts">
    import { goto } from '$app/navigation';

    import * as noteStore from '$lib/stores/note';
    import { onDestroy } from 'svelte';
    import ThreeDotsVertical from '../../../icons/three-dots-vertical.svelte';

    export let folderId: number | null = null;
    export let noteId: number | null = null;

    let title: string | null = null;
    let text = '';
    let modified = false;
    let deleting = false;
    let showActionMenu = false;

    if (noteId) {
        noteStore.note(noteId).then((n) => {
            if (n) {
                title = n.title;
                text = n.text;
                folderId = n.folderId;
            }
        });
    }

    onDestroy(() => {
        if (deleting) {
            return;
        }

        if (noteId !== null && modified) {
            noteStore.updateNote(folderId!, noteId, text);
        } else if (noteId === null && text.trim().length !== 0) {
            if (!folderId) {
                throw new Error('Either folderId or noteId must be provided');
            }

            noteStore.createNote(folderId, text);
        }
    });

    function deleteNote() {
        if (deleting) {
            return;
        }

        deleting = true;

        if (noteId === null) {
            goto(`/app/folder/${folderId}`);
            return;
        }

        noteStore
            .deleteNote(folderId!, noteId)
            .then(() => goto(`/app/folder/${folderId}`))
            .catch(() => (deleting = false));
    }
</script>
<div class="my-2 p-3 d-flex flex-column" style="height: calc(100% - 1rem);">
    <div class="d-flex pb-2 mb-4">
        <div class="flex-grow-1">
            <h3 class="d-inline-block m-0">{title ?? 'New Note'}</h3>
            <small class="text-black-50">Note</small>
        </div>
        <div class="dropdown">
            <button class="btn btn-light" on:click={() => showActionMenu = !showActionMenu}><ThreeDotsVertical/></button>
            <ul class="dropdown-menu position-absolute end-0 mt-1" class:show={showActionMenu}>
                <li>
                    <button class="dropdown-item text-danger text-center" on:click={() => deleteNote()}>
                        {#if deleting}
                            <span class="spinner-border spinner-border-sm me-2" role="status" aria-hidden="true"></span>
                        {/if}
                        Delete Note
                    </button>
                </li>
            </ul>
        </div>
    </div>


    <textarea class="flex-grow-1 border rounded p-3" style="resize: none;" on:input={() => (modified = true)} bind:value={text}/>
</div>

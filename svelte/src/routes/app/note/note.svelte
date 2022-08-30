<script lang="ts">
    import { goto } from '$app/navigation';

    import * as noteStore from '$lib/stores/note';
    import { onDestroy } from 'svelte';

    export let folderId: number | null = null;
    export let noteId: number | null = null;

    let title: string | null = null;
    let text = '';
    let modified = false;
    let deleting = false;

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

<h1>{title ?? 'New Note'}</h1>
<button on:click={() => deleteNote()}>Delete</button>

<textarea on:input={() => (modified = true)} bind:value={text} />

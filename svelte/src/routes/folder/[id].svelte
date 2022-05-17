<script lang="ts">
    import { notes as fetchNotes, create_note as createNote } from 'mavinote-wasm';
    import init from '$lib/wasm';
    import { onMount } from 'svelte';
    import { page } from '$app/stores';
    import { goto } from '$app/navigation';

    let notes: any[] = [];
    let inProgress = false;

    onMount(async () => {
        await init();
        try {
            notes = JSON.parse(await fetchNotes(parseInt($page.params.id)));
        } catch (e) {
            console.error('error in 3, 2, 1 ...', e);
        }
    });

    function create() {
        if (inProgress) {
            return;
        }

        inProgress = true;

        createNote(parseInt($page.params.id))
            .then((noteId) => goto(`/note/${noteId}`))
            .catch((e: any) => console.error('failed to create note', e))
            .finally(() => (inProgress = false));
    }
</script>

<div>
    <h1>Notes</h1>
    <ul>
        {#each notes as note}
            <li><a href={`/note/${note.id}`}>{note.title}</a></li>
        {/each}
    </ul>
    <button on:click={create}>Create</button>
</div>

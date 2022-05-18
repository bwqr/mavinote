<script lang="ts">
    import * as noteStore from '$lib/stores/note';
    import init from '$lib/wasm';
    import { onMount } from 'svelte';
    import { page } from '$app/stores';
    import { goto } from '$app/navigation';
    import type { Note } from '$lib/models';

    let notes: Note[] = [];
    let inProgress = false;

    onMount(async () => {
        await init();
        notes = await noteStore.notes(parseInt($page.params.id));
    });

    function create() {
        if (inProgress) {
            return;
        }

        inProgress = true;

        noteStore
            .createNote(parseInt($page.params.id))
            .then((noteId) => goto(`/note/${noteId}`))
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

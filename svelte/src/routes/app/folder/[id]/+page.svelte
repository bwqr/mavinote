<script lang="ts">
    import { goto } from '$app/navigation';

    import { page } from '$app/stores';
    import type { Note } from '$lib/models';
    import * as noteStore from '$lib/stores/note';
    import { onDestroy } from 'svelte';

    let notes: Note[] = [];
    let deleting = false;

    const sub = page.subscribe(({ params }) => {
        notes = [];
        noteStore.notes(parseInt(params.id)).then((n) => (notes = n));
    });

    onDestroy(sub);

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

<button on:click={() => deleteFolder()}>Delete Folder</button>

<a href={`/app/folder/${$page.params.id}/create-note`}>Create Note</a>

<ol>
    {#each notes as note}
        <li><a href={`/app/note/${note.id}`}>{note.title ?? 'New Note'}</a></li>
    {/each}
</ol>

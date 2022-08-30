<script lang="ts">
    import { goto } from '$app/navigation';

    import { page } from '$app/stores';
    import type { Note } from '$lib/models';
    import * as noteStore from '$lib/stores/note';
    import { Subscription } from 'rxjs';
    import { onDestroy } from 'svelte';

    let notes: Note[] = [];
    let deleting = false;
    let subs = new Subscription();

    const sub = page.subscribe(({ params }) => {
        notes = [];
        subs.add(noteStore.notes(parseInt(params.id)).subscribe((n) => (notes = n)));
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

<button on:click={() => deleteFolder()}>Delete Folder</button>

<a href={`/app/folder/${$page.params.id}/create-note`}>Create Note</a>

<ol>
    {#each notes as note}
        <li><a href={`/app/note/${note.id}`}>{note.title ?? 'New Note'}</a></li>
    {/each}
</ol>

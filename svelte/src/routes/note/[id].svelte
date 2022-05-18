<script lang="ts">
    import init from '$lib/wasm';
    import * as noteStore from '$lib/stores/note';
    import { onDestroy, onMount } from 'svelte';
    import { page } from '$app/stores';
    import type { Note } from '$lib/models';

    let note: Note | undefined = undefined;

    onMount(async () => {
        await init();
        note = await noteStore.note(parseInt($page.params.id));
    });

    onDestroy(() => {
        if (note) {
            noteStore
                .updateNote(note.id, note.text)
                .catch((e: any) => console.error('failed to update note', e));
        }
    });

    function updateText(event: Event) {
        if (note) {
            note.text = (event.target as HTMLTextAreaElement).value;
        }
    }
</script>

<div>
    <h1>Notes</h1>
    {#if note}
        <h3>{note.title}</h3>
        <textarea on:change={updateText}>{note.text}</textarea>
    {/if}
</div>

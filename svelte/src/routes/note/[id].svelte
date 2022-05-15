<script lang="ts">
    import init, { note as fetchNote, update_note as updateNote } from 'mavinote-wasm';
    import { onDestroy, onMount } from 'svelte';
    import { page } from '$app/stores';

    let note: any = null;

    onMount(async () => {
        await init();
        try {
            note = JSON.parse(await fetchNote(parseInt($page.params.id)));
        } catch (e) {
            console.error('error in 3, 2, 1 ...', e);
        }
    });

    onDestroy(async () => {
        if (note) {
            try {
                await updateNote(note.id, note.text)
            } catch(e: any) {
                console.error('failed to update note', e);
            }
        }
    });

    function updateText(event: Event) {
        note.text = (event.target as HTMLTextAreaElement).value;
    }
</script>

<div>
    <h1>Notes</h1>
    {#if note }
        <h3>{note.title}</h3>
        <textarea on:change={updateText}>{note.text}</textarea>
    {/if}
</div>

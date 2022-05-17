<script lang="ts">
    import init from '$lib/wasm';
    import { note as fetchNote, update_note as updateNote } from 'mavinote-wasm';
    import { onDestroy, onMount } from 'svelte';
    import { page } from '$app/stores';

    let inProgress = false;
    let note: any = null;

    onMount(async () => {
        await init();
        try {
            note = JSON.parse(await fetchNote(parseInt($page.params.id)));
        } catch (e) {
            console.error('error in 3, 2, 1 ...', e);
        }
    });

    onDestroy(() => {
        if (note) {
            updateNote(note.id, note.text)
                .catch((e: any) => console.error('failed to update note', e));
        }
    });

    function update() {
        if (inProgress) {
            return;
        }

        inProgress = true;

        updateNote(note.id, note.text)
            .then(() => console.log('updated'))
            .catch((e: any) => console.log('failed to update', e))
            .finally(() => inProgress = false);
    }

    function updateText(event: Event) {
        note.text = (event.target as HTMLTextAreaElement).value;
    }
</script>

<div>
    <h1>Notes</h1>
    {#if note }
        <h3>{note.title}</h3>
        <textarea on:change={updateText}>{note.text}</textarea>
        <button on:click={update}>Update</button>
    {/if}
</div>

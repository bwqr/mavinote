<script lang="ts">
    import { page } from '$app/stores';
    import { onDestroy } from 'svelte';
    import Note from '../note.svelte';

    let noteId: number | null = null;

    const sub = page.subscribe(({ params }) => (noteId = parseInt(params.id)));

    onDestroy(sub);
</script>

{#if noteId}
    <Note {noteId} />
{/if}

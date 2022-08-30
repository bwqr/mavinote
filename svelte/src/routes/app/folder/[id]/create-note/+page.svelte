<script lang="ts">
    import { page } from '$app/stores';
    import Note from '../../../note/note.svelte';
</script>

<Note folderId={parseInt($page.params.id)} />

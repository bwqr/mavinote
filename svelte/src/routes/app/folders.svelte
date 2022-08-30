<script lang="ts">
    import type { Folder } from '$lib/models';
    import * as noteStore from '$lib/stores/note';

    let folders: Folder[] = [];

    noteStore.folders().then((f) => (folders = f));
</script>

<h1>Folders</h1>
<ul>
    {#each folders as folder}
        <li><a href={`/folder/${folder.id}`}>{folder.name}</a></li>
    {/each}
</ul>

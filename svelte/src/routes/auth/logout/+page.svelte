<script lang="ts">
    import init from '$lib/wasm';
    import { logout } from '$lib/stores/auth';
    import { onMount } from 'svelte';
    import { goto } from '$app/navigation';

    onMount(async () => {
        await init();

        logout();

        await goto('/auth/login')

        window.location.reload();
    });
</script>

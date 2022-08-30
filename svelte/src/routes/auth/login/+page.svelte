<script lang="ts">
    import { goto } from '$app/navigation';
    import { onMount } from 'svelte';
    import * as authStore from '$lib/stores/auth';
    import init from '$lib/wasm';

    let email = '';
    let password = '';
    let inProgress = false;

    onMount(() => init());

    function login() {
        if (inProgress) {
            return;
        }

        inProgress = true;

        authStore
            .login(email, password)
            .then(() => goto('/app'))
            .finally(() => (inProgress = false));
    }
</script>

<form on:submit|preventDefault={login}>
    <input type="email" placeholder="Email" bind:value={email} required />
    <input type="password" placeholder="Password" bind:value={password} required />
    <button type="submit">Login</button>
</form>

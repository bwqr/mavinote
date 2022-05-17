<script lang="ts">
    import { goto } from '$app/navigation';
    import { onMount } from 'svelte';
    import init from '../lib/wasm';

    import * as authStore from '../stores/auth';

    let email = '';
    let password = '';
    let inProgress = false;

    onMount(async () => init());

    function login() {
        if (inProgress) {
            return;
        }

        inProgress = true;

        authStore
            .login(email, password)
            .then(() => goto('/'))
            .finally(() => (inProgress = false));
    }
</script>

<h1>Login</h1>

<form on:submit|preventDefault={login}>
    <input type="email" placeholder="Email" bind:value={email} required />
    <input type="password" placeholder="Password" bind:value={password} required />
    <button type="submit">Login</button>
</form>

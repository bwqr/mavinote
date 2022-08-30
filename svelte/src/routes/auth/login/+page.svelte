<script lang="ts">
    import { goto } from '$app/navigation';
    import { onMount } from 'svelte';
    import * as authStore from '$lib/stores/auth';
    import init from '$lib/wasm';
    import { Unauthorized } from '$lib/models';

    let email = '';
    let password = '';
    let error: string | undefined = undefined;
    let inProgress = false;

    onMount(() => init());

    function login() {
        if (inProgress) {
            return;
        }

        error = undefined;
        inProgress = true;

        authStore
            .login(email, password)
            .then(() => goto('/app'))
            .catch((e) => {
                if (e instanceof Unauthorized) {
                    error = 'Invalid credentials';
                    return;
                }

                return Promise.reject(e);
            })
            .finally(() => (inProgress = false));
    }
</script>

<form on:submit|preventDefault={login}>
    <input type="email" placeholder="Email" bind:value={email} required />
    <input type="password" placeholder="Password" bind:value={password} required />
    {#if error}
        <span>{error}</span>
    {/if}
    <button type="submit">Login</button>
</form>

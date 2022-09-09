<script lang="ts">
    import { goto } from '$app/navigation';
    import { onMount } from 'svelte';
    import * as authStore from '$lib/stores/auth';
    import init from '$lib/wasm';
    import { MessageError } from '$lib/models';

    let name = '';
    let email = '';
    let password = '';
    let error: string | undefined = undefined;
    let inProgress = false;

    onMount(() => init());

    function signUp() {
        if (inProgress) {
            return;
        }

        error = undefined;
        inProgress = true;

        authStore
            .signUp(name, email, password)
            .then(() => goto('/app'))
            .catch((e) => {
                if (e instanceof MessageError) {
                    error = e.message;
                    return;
                }

                return Promise.reject(e);
            })
            .finally(() => (inProgress = false));
    }
</script>


<div class="p-4 d-flex justify-content-center align-items-center" style="height: 100vh; background-color: var(--bs-gray-300);">
    <form on:submit|preventDefault={signUp} class="shadow-sm bg-body px-4 py-5 rounded" style="max-width: 350px">
        <div class="mb-3">
            <h4>Sign Up</h4>
        </div>
        <div class="mb-3">
            <input class="form-control" type="text" placeholder="Name" bind:value={name} required />
        </div>
        <div class="mb-3">
            <input class="form-control" type="email" placeholder="Email" bind:value={email} required />
        </div>
        <div class="mb-3">
            <input class="form-control" type="password" placeholder="Password" bind:value={password} required />
        </div>

        {#if error}
            <p class="text-end mb-2 text-danger">{error}</p>
        {/if}

        <button class="btn btn-primary w-100 mb-2" type="submit">
            {#if inProgress}
                <span class="spinner-border spinner-border-sm me-2" role="status" aria-hidden="true"></span>
            {/if}
            Sign Up
        </button>

        <small>Already have an account? <a href="/auth/login">Login</a></small>
    </form>
</div>

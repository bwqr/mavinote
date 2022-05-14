import adapter from '@sveltejs/adapter-auto';
import preprocess from 'svelte-preprocess';

/** @type {import('@sveltejs/kit').Config} */
const config = {
    // Consult https://github.com/sveltejs/svelte-preprocess
    // for more information about preprocessors
    preprocess: preprocess(),

    kit: {
        adapter: adapter(),
        vite: {
            server: {
                fs: {
                    allow: ['/mnt/storage/projects/mavinote/wasm/pkg/'],
                },
            },

        },
    },
};

export default config;

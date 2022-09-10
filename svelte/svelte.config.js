import adapter from '@sveltejs/adapter-node';
import preprocess from 'svelte-preprocess';

/** @type {import('@sveltejs/kit').Config} */
const config = {
    // Consult https://github.com/sveltejs/svelte-preprocess
    // for more information about preprocessors
    preprocess: preprocess(),
    kit: {
        adapter: adapter(),
        alias: {
            '$components': 'src/components',
            '$components/*': 'src/components/*',
        },
    },
};

export default config;

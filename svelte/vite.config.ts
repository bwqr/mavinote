import { sveltekit } from '@sveltejs/kit/vite';
import type { UserConfig } from 'vite';

const config: UserConfig = {
    plugins: [sveltekit()],
    server: {
        fs: {
            allow: ['../reax/wasm/pkg'],
        }
    },
};

export default config;

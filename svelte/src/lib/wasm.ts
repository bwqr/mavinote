import init, { type InitOutput } from 'mavinote-wasm';

let initOutputs: InitOutput | undefined = undefined;

export default async function (): Promise<InitOutput> {
    if (initOutputs) {
        return Promise.resolve(initOutputs);
    }

    initOutputs = await init();

    return initOutputs;
}

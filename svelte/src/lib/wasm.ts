import init, { init_wasm, abort_stream, type InitOutput } from 'mavinote-wasm';
import { ReaxError } from './models';
import { BincodeDeserializer } from './serde/bincode/bincodeDeserializer';
import type { Deserializer } from './serde/serde/deserializer';

let initOutputs: InitOutput | undefined = undefined;

export class Stream {
    private _joinHandle: number | undefined = undefined

    constructor(
        private onNext: (deserializer: Deserializer) => void,
        private onError: (error: ReaxError) => void,
        private onComplete: () => void,
        private onStart: (streamId: number) => number,
    ) {
    }

    handle(bytes: Uint8Array) {
        const deserializer = new BincodeDeserializer(bytes);

        const index = deserializer.deserializeVariantIndex();

        switch (index) {
            case 0:
                return this.onNext(deserializer);
            case 1:
                return this.onError(ReaxError.deserialize(deserializer));
            case 2:
                return this.onComplete();
            default:
                throw new Error('Unknown index on Stream');
        }
    }

    start(streamId: number) {
        if (this._joinHandle) {
            throw new Error('Stream started more than once');
        }

        this._joinHandle = this.onStart(streamId);
    }

    joinHandle(): number | undefined {
        return this._joinHandle;
    }
}

export class Runtime {
    private static readonly MAX_UINT_32 = 2 ** 32 - 1;

    static instance: Runtime = new Runtime();

    static handleStream(streamId: number, bytes: Uint8Array) {
        console.log(`handling the stream with streamId ${streamId} and ${bytes.length} lenght of bytes`);

        const stream = Runtime.instance.streams.get(streamId);

        if (!stream) {
            throw new Error('Unknown stream is received');
        }

        stream.handle(bytes);
    }

    private streams: Map<number, Stream> = new Map();

    private constructor() { }

    startStream(stream: Stream): number {
        let streamId = Math.trunc(Math.random() * Runtime.MAX_UINT_32);

        while (this.streams.has(streamId)) {
            streamId = Math.trunc(Math.random() * Runtime.MAX_UINT_32);
        }

        this.streams.set(streamId, stream);

        stream.start(streamId);

        return streamId;
    }

    abortStream(streamId: number) {
        const stream = this.streams.get(streamId);

        if (!stream) {
            throw new Error('Unknown stream is being tried to abort');
        }

        this.streams.delete(streamId);

        const joinHandle = stream.joinHandle();
        if (!joinHandle) {
            console.warn('aborting a stream which have not started yet', streamId);
            return;
        }

        abort_stream(joinHandle);
    }
}

(globalThis as any).WasmRuntime = Runtime;

export default async function(): Promise<InitOutput> {
    if (initOutputs) {
        return initOutputs;
    }

    initOutputs = await init();

    init_wasm(globalThis.process && process.env['API_URL'] ? process.env['API_URL'] : import.meta.env.VITE_API_URL);

    return initOutputs;
}

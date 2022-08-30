import { goto } from "$app/navigation";
import { BincodeDeserializer } from "$lib/serde/bincode/bincodeDeserializer";
import type { Deserializer } from "$lib/serde/serde/deserializer";
import { ReaxError, Unauthorized } from "../models";

export function decodeAndHandleError(buffer: Uint8Array): Promise<any> {
    const error = ReaxError.deserialize(new BincodeDeserializer(buffer));

    handleError(error);

    return Promise.reject(error);
}

export function handleError(error: ReaxError) {
    if (error instanceof Unauthorized) {
        goto('/auth/login');
    } else {
        console.warn('Unhandled error', error);
    };
}

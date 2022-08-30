import { goto } from "$app/navigation";
import { BincodeDeserializer } from "$lib/serde/bincode/bincodeDeserializer";
import { ReaxError, Unauthorized } from "../models";

export function handleError(buffer: Uint8Array): Promise<any> {
    const error = ReaxError.deserialize(new BincodeDeserializer(buffer));

    if (error instanceof Unauthorized) {
        goto('/auth/login');
    }

    return Promise.reject(error);
}

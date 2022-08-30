import { ReaxError } from '$lib/models';
import { BincodeDeserializer } from '$lib/serde/bincode/bincodeDeserializer';
import * as wasm from 'mavinote-wasm';

export async function login(email: string, password: string) {
    return wasm.auth_login(email, password)
        .catch(buffer => Promise.reject(ReaxError.deserialize(new BincodeDeserializer(buffer))));
}

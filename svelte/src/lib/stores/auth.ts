import * as wasm from 'mavinote-wasm';
import { handleError } from '.';

export function login(email: string, password: string): Promise<void> {
    return wasm.auth_login(email, password)
        .catch(handleError);
}

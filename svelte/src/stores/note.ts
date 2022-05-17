import type { Folder } from "../models";
import * as wasm from 'mavinote-wasm';
import { handleError } from ".";

export function folders(): Promise<Folder[]> {
    return wasm.note_folders()
        .then(f => JSON.parse(f))
        .catch(handleError);
}

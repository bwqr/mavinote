import { deserializeFolder, type Folder } from "../models";
import * as noteWasm from "mavinote-wasm";
import { deserializeVec } from '$lib/serde';
import { BincodeDeserializer } from "$lib/serde/bincode/bincodeDeserializer";
import { handleError } from ".";

export async function folders(): Promise<Folder[]> {
    return noteWasm.note_folders()
        .then(buffer => deserializeVec(new BincodeDeserializer(buffer), d => deserializeFolder(d)))
        .catch(handleError);
}

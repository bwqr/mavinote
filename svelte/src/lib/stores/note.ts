import { deserializeFolder, deserializeNote, State, type Folder, type Note } from "../models";
import * as noteWasm from "mavinote-wasm";
import { deserializeOption, deserializeVec } from '$lib/serde';
import { BincodeDeserializer } from "$lib/serde/bincode/bincodeDeserializer";
import { decodeAndHandleError, handleError } from ".";
import { Observable } from "rxjs";
import { Runtime, Stream } from "$lib/wasm";

export function folders(): Observable<Folder[]> {
    return new Observable((sub) => {
        const streamId = Runtime.instance.startStream(new Stream(
            (deserializer) => sub.next(deserializeVec(deserializer, (d) => deserializeFolder(d))),
            (error) => {
                handleError(error);
                sub.error(error);
            },
            () => sub.complete(),
            (streamId) => noteWasm.note_folders(streamId),
        ));

        return {
            unsubscribe: () => Runtime.instance.abortStream(streamId),
        };
    });
}

export async function createFolder(folderName: string): Promise<Folder> {
    return noteWasm.note_create_folder(folderName)
        .then(buffer => deserializeFolder(new BincodeDeserializer(buffer)))
        .catch(decodeAndHandleError);
}

export async function deleteFolder(folderId: number): Promise<void> {
    return noteWasm.note_delete_folder(folderId)
        .then(() => { })
        .catch(decodeAndHandleError);
}

export async function notes(folderId: number): Promise<Note[]> {
    return noteWasm.note_notes(folderId)
        .then(buffer => {
            const notes = deserializeVec(new BincodeDeserializer(buffer), d => deserializeNote(d));

            return notes.filter(n => n.state === State.Clean);
        })
        .catch(decodeAndHandleError);
}

export async function note(noteId: number): Promise<Note | null> {
    return noteWasm.note_note(noteId)
        .then(buffer => deserializeOption(new BincodeDeserializer(buffer), d => deserializeNote(d)))
        .catch(decodeAndHandleError);
}

export async function createNote(folderId: number, text: string): Promise<Note> {
    return noteWasm.note_create_note(folderId, text)
        .then(buffer => deserializeNote(new BincodeDeserializer(buffer)))
        .catch(decodeAndHandleError);
}

export async function updateNote(noteId: number, text: string): Promise<void> {
    return noteWasm.note_update_note(noteId, text)
        .then(() => { })
        .catch(decodeAndHandleError);
}

export async function deleteNote(noteId: number): Promise<void> {
    return noteWasm.note_delete_note(noteId)
        .then(() => { })
        .catch(decodeAndHandleError);
}

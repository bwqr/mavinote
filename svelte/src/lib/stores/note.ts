import type { Folder, Note } from "../models";
import * as wasm from 'mavinote-wasm';
import { handleError } from ".";

export function folders(): Promise<Folder[]> {
    return wasm.note_folders()
        .then(str => JSON.parse(str))
        .catch(handleError);
}

export function createFolder(name: string): Promise<void> {
    return wasm.note_create_folder(name)
        .catch(handleError);
}

export function notes(folderId: number): Promise<Note[]> {
    return wasm.note_notes(folderId)
        .then(str => JSON.parse(str))
        .catch(handleError);
}

export function createNote(folderId: number): Promise<number> {
    return wasm.note_create_note(folderId)
        .catch(handleError);
}

export function note(noteId: number): Promise<Note> {
    return wasm.note_note(noteId)
        .then(str => JSON.parse(str))
        .catch(handleError);
}

export function updateNote(noteId: number, text: string): Promise<void> {
    return wasm.note_update_note(noteId, text)
        .catch(handleError);
}

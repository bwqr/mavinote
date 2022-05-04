package com.bwqr.mavinote.viewmodels

import com.bwqr.mavinote.models.Folder
import com.bwqr.mavinote.models.Note
import com.bwqr.mavinote.models.TraitHelpers
import kotlin.coroutines.suspendCoroutine


class NoteViewModel {
    suspend fun folders(): Result<List<Folder>> {
        return suspendCoroutine { cont ->
            val waitId = Runtime.instance.wait(AsyncWait(cont) { deserializer ->
                TraitHelpers.deserializeList(deserializer) { Folder.deserialize(it) }
            })

            _folders(waitId)
        }
    }

    suspend fun addFolder(name: String): Result<Unit> {
        return suspendCoroutine { cont ->
            val waitId = Runtime.instance.wait(AsyncWait(cont) { })

            _addFolder(waitId, name)
        }
    }

    suspend fun notes(folderId: Int): Result<List<Note>> {
        return suspendCoroutine { cont ->
            val waitId = Runtime.instance.wait(AsyncWait(cont) { deserializer ->
                TraitHelpers.deserializeList(deserializer) { Note.deserialize(it) }
            })

            _noteSummaries(waitId, folderId)
        }
    }

    suspend fun note(noteId: Int): Result<Note?> {
        return suspendCoroutine { cont ->
            val waitId = Runtime.instance.wait(AsyncWait(cont) { deserializer ->
                TraitHelpers.deserializeOption(deserializer) { Note.deserialize(it) }
            })

            _note(waitId, noteId)
        }
    }

    suspend fun createNote(folderId: Int): Result<Int> {
        return suspendCoroutine { cont ->
            val waitId = Runtime.instance.wait(AsyncWait(cont) { deserializer ->
                deserializer.deserialize_i32()
            })

            _createNote(waitId, folderId)
        }
    }

    suspend fun updateNote(noteId: Int, text: String): Result<Unit> {
        return suspendCoroutine { cont ->
            val waitId = Runtime.instance.wait(AsyncWait(cont) { })

            _updateNote(waitId, noteId, text)
        }
    }

    private external fun _folders(subId: Int)
    private external fun _addFolder(subId: Int, name: String)
    private external fun _noteSummaries(subId: Int, folderId: Int)
    private external fun _note(subId: Int, noteId: Int)
    private external fun _createNote(subId: Int, folderId: Int): Int
    private external fun _updateNote(subId: Int, noteId: Int, text: String)
}
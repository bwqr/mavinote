package com.bwqr.mavinote.viewmodels

import com.bwqr.mavinote.models.Folder
import com.bwqr.mavinote.models.Note
import com.bwqr.mavinote.reax.Runtime
import com.bwqr.mavinote.reax.deserializeList
import com.bwqr.mavinote.reax.deserializeOption
import kotlinx.coroutines.flow.Flow

class NoteViewModel {
    companion object {
        fun init() = _init()

        suspend fun sync(): Unit = Runtime.runUnitOnce { _sync(it) }


        suspend fun addDevice(accountId: Int, fingerprint: String): Unit =
            Runtime.runUnitOnce { _addDevice(it, accountId, fingerprint) }

        fun folders(): Flow<List<Folder>> = Runtime.runStream({
            deserializeList(it) { deserializer ->
                Folder.deserialize(deserializer)
            }
        }, { _folders(it) })

        suspend fun folder(folderId: Int): Folder? = Runtime.runOnce({
            deserializeOption(it) { deserializer ->
                Folder.deserialize(deserializer)
            }
        }, { _folder(it, folderId) })

        suspend fun createFolder(accountId: Int, name: String): Unit =
            Runtime.runUnitOnce { _createFolder(it, accountId, name) }

        suspend fun deleteFolder(folderId: Int): Unit =
            Runtime.runUnitOnce { _deleteFolder(it, folderId) }

        fun notes(folderId: Int): Flow<List<Note>> = Runtime.runStream({
            deserializeList(it) { deserializer ->
                Note.deserialize(deserializer)
            }
        }, { _noteSummaries(it, folderId) })

        suspend fun note(noteId: Int): Note? = Runtime.runOnce({
            deserializeOption(it) { deserializer ->
                Note.deserialize(deserializer)
            }
        }, { _note(it, noteId) })

        suspend fun createNote(folderId: Int, text: String): Int =
            Runtime.runOnce({ it.deserialize_i32() }) { _createNote(it, folderId, text) }

        suspend fun updateNote(noteId: Int, text: String): Unit =
            Runtime.runUnitOnce { _updateNote(it, noteId, text) }

        suspend fun deleteNote(noteId: Int): Unit = Runtime.runUnitOnce { _deleteNote(it, noteId) }
    }
}

private external fun _init()

private external fun _sync(onceId: Int): Long
private external fun _addDevice(onceId: Int, accountId: Int, fingerprint: String): Long
private external fun _folders(streamId: Int): Long
private external fun _folder(onceId: Int, folderId: Int): Long
private external fun _createFolder(onceId: Int, accountId: Int, name: String): Long
private external fun _deleteFolder(onceId: Int, folderId: Int): Long
private external fun _noteSummaries(streamId: Int, folderId: Int): Long
private external fun _note(onceId: Int, noteId: Int): Long
private external fun _createNote(onceId: Int, folderId: Int, text: String): Long
private external fun _updateNote(onceId: Int, noteId: Int, text: String): Long
private external fun _deleteNote(onceId: Int, noteId: Int): Long
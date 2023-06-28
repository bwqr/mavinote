package com.bwqr.mavinote.viewmodels

import com.bwqr.mavinote.models.Folder
import com.bwqr.mavinote.models.Note
import com.bwqr.mavinote.reax.DeInt
import com.bwqr.mavinote.reax.DeList
import com.bwqr.mavinote.reax.DeOption
import com.bwqr.mavinote.reax.Runtime
import kotlinx.coroutines.flow.Flow

class NoteViewModel {
    companion object {
        fun init() = _init()

        suspend fun sync(): Unit = Runtime.runOnceUnit { _sync(it) }

        fun folders(): Flow<List<Folder>> =
            Runtime.runStream(DeList(Folder.Companion)) { _folders(it) }

        suspend fun folder(folderId: Int): Folder? =
            Runtime.runOnce(DeOption(Folder.Companion)) { _folder(it, folderId) }

        suspend fun createFolder(accountId: Int, name: String): Unit =
            Runtime.runOnceUnit { _createFolder(it, accountId, name) }

        suspend fun deleteFolder(folderId: Int): Unit =
            Runtime.runOnceUnit { _deleteFolder(it, folderId) }

        fun notes(folderId: Int): Flow<List<Note>> =
            Runtime.runStream(DeList(Note.Companion)) { _noteSummaries(it, folderId) }

        suspend fun note(noteId: Int): Note? =
            Runtime.runOnce(DeOption(Note.Companion)) { _note(it, noteId) }

        suspend fun createNote(folderId: Int, text: String): Int =
            Runtime.runOnce(DeInt) { _createNote(it, folderId, text) }

        suspend fun updateNote(noteId: Int, text: String): Unit =
            Runtime.runOnceUnit { _updateNote(it, noteId, text) }

        suspend fun deleteNote(noteId: Int): Unit =
            Runtime.runOnceUnit { _deleteNote(it, noteId) }
    }
}

private external fun _init()
private external fun _sync(onceId: Int): Long
private external fun _folders(streamId: Int): Long
private external fun _folder(onceId: Int, folderId: Int): Long
private external fun _createFolder(onceId: Int, accountId: Int, name: String): Long
private external fun _deleteFolder(onceId: Int, folderId: Int): Long
private external fun _noteSummaries(streamId: Int, folderId: Int): Long
private external fun _note(onceId: Int, noteId: Int): Long
private external fun _createNote(onceId: Int, folderId: Int, text: String): Long
private external fun _updateNote(onceId: Int, noteId: Int, text: String): Long
private external fun _deleteNote(onceId: Int, noteId: Int): Long
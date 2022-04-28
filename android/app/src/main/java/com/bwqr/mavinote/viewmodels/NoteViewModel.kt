package com.bwqr.mavinote.viewmodels

import com.bwqr.mavinote.models.Folder
import com.bwqr.mavinote.models.Note
import com.bwqr.mavinote.models.NoteSummary
import com.bwqr.mavinote.models.listDeserialize
import com.novi.bincode.BincodeDeserializer


class NoteViewModel {
    fun folders(): List<Folder> {
        return listDeserialize(_folders()) { Folder.deserialize(it) }
    }

    fun notes(folderId: Int): List<NoteSummary> {
        return listDeserialize(_noteSummaries(folderId)) { NoteSummary.deserialize(it) }
    }

    fun note(noteId: Int): Note? {
        val bytes = _note(noteId)

        if (bytes.isEmpty()) {
            return null
        }

        return Note.deserialize(BincodeDeserializer(bytes))
    }

    fun updateNote(noteId: Int, text: String) {
        _updateNote(noteId, text)
    }

    private external fun _folders(): ByteArray
    private external fun _noteSummaries(folderId: Int): ByteArray
    private external fun _note(noteId: Int): ByteArray
    private external fun _updateNote(noteId: Int, text: String)
}
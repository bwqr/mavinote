package com.bwqr.mavinote.models

data class Note constructor(
    val id: Int,
    val folderId: Int,
    val title: String,
    val text: String,
) {
    fun cloneWithText(text: String): Note {
        return Note(
            id,
            folderId,
            title,
            text
        )
    }
}
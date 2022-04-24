package com.bwqr.mavinote.models

data class NoteSummary constructor(
    val id: Int,
    val folderId: Int,
    val title: String,
    val summary: String,
)
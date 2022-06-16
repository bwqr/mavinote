package com.bwqr.mavinote.ui.note

import androidx.compose.foundation.layout.Column
import androidx.compose.material.Text
import androidx.compose.material.TextField
import androidx.compose.runtime.*
import androidx.compose.ui.platform.LocalLifecycleOwner
import androidx.lifecycle.Lifecycle
import androidx.lifecycle.LifecycleEventObserver
import com.bwqr.mavinote.models.Note
import com.bwqr.mavinote.models.ReaxException
import com.bwqr.mavinote.viewmodels.NoteViewModel
import kotlinx.coroutines.GlobalScope
import kotlinx.coroutines.launch

@Composable
fun Note(noteId: Int) {
    val lifecycleOwner = LocalLifecycleOwner.current

    var note: Note? by remember {
        mutableStateOf(null)
    }

    var text by remember {
        mutableStateOf("")
    }

    LaunchedEffect(key1 = 1) {
        try {
            NoteViewModel().note(noteId)?.let {
                note = it
                text = it.text
            }
        } catch (e: ReaxException) {
            e.handle()
        }
    }

    Column {
        note?.let {
            Text(text = it.title ?: "New Note")
        }

        TextField(value = text, onValueChange = { text = it })
    }

    DisposableEffect(lifecycleOwner) {
        val observer = LifecycleEventObserver { _, event ->
            when (event) {
                Lifecycle.Event.ON_STOP -> GlobalScope.launch {
                    try {
                        NoteViewModel().updateNote(note!!.id, note!!.folderId, text)
                    } catch (e: ReaxException) {
                        e.handle()
                    }
                }
                else -> {}
            }
        }

        lifecycleOwner.lifecycle.addObserver(observer)

        onDispose {
            lifecycleOwner.lifecycle.removeObserver(observer)
        }
    }
}
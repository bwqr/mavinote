package com.bwqr.mavinote.ui.note

import androidx.compose.material.TextField
import androidx.compose.runtime.*
import androidx.compose.ui.platform.LocalLifecycleOwner
import androidx.lifecycle.Lifecycle
import androidx.lifecycle.LifecycleEventObserver
import com.bwqr.mavinote.models.Note
import com.bwqr.mavinote.viewmodels.NoteViewModel
import kotlinx.coroutines.launch

@Composable
fun Note(noteId: Int) {
    val lifecycleOwner = LocalLifecycleOwner.current

    val scope = rememberCoroutineScope()

    var note: Note? by remember {
        mutableStateOf(null)
    }

    var text by remember {
        mutableStateOf("")
    }

    LaunchedEffect(key1 = 1) {
        NoteViewModel().note(noteId).getOrThrow()?.let {
            note = it
            text = it.text
        }
    }

    TextField(value = text, onValueChange = { text = it })

    DisposableEffect(lifecycleOwner) {
        val observer = LifecycleEventObserver { _, event ->
            when (event) {
                Lifecycle.Event.ON_STOP -> scope.launch {
                    NoteViewModel().updateNote(note!!.id, text).getOrThrow()
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
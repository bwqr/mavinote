package com.bwqr.mavinote.ui

import android.util.Log
import androidx.compose.material.Text
import androidx.compose.material.TextField
import androidx.compose.runtime.*
import androidx.compose.ui.platform.LocalLifecycleOwner
import androidx.lifecycle.Lifecycle
import androidx.lifecycle.LifecycleEventObserver
import com.bwqr.mavinote.viewmodels.NoteViewModel

@Composable
fun Note(noteId: Int) {
    val lifecycleOwner = LocalLifecycleOwner.current

    val note = remember {
        NoteViewModel().note(noteId)!!
    }

    var text by remember {
        mutableStateOf(note.text)
    }

    TextField(value = text, onValueChange = { text = it })
    
    DisposableEffect(lifecycleOwner) {
        val observer = LifecycleEventObserver { _, event ->
            when (event) {
                Lifecycle.Event.ON_STOP -> NoteViewModel().updateNote(note.id, text)
                else -> {}
            }
        }

        lifecycleOwner.lifecycle.addObserver(observer)

        onDispose {
            lifecycleOwner.lifecycle.removeObserver(observer)
        }
    }
}
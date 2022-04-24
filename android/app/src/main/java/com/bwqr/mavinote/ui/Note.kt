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

    var title by remember {
        mutableStateOf(note.title)
    }
    var text by remember {
        mutableStateOf(note.text)
    }

    TextField(value = text, onValueChange = { text = it })
    
    DisposableEffect(lifecycleOwner) {
        Log.d("Note", "DisposableEffectScope is launched")

        val observer = LifecycleEventObserver { _, event ->
            Log.d("Note", "LifecylceEvent observed")
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
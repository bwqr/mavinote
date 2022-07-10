package com.bwqr.mavinote.ui.note

import androidx.compose.foundation.layout.Column
import androidx.compose.material.Button
import androidx.compose.material.Text
import androidx.compose.material.TextField
import androidx.compose.runtime.*
import androidx.compose.ui.platform.LocalLifecycleOwner
import androidx.lifecycle.Lifecycle
import androidx.lifecycle.LifecycleEventObserver
import androidx.navigation.NavController
import com.bwqr.mavinote.models.Note
import com.bwqr.mavinote.models.ReaxException
import com.bwqr.mavinote.viewmodels.NoteViewModel
import kotlinx.coroutines.GlobalScope
import kotlinx.coroutines.launch

@Composable
fun Note(navController: NavController, noteId: Int) {
    val lifecycleOwner = LocalLifecycleOwner.current
    val coroutineScope = rememberCoroutineScope()

    var updateNote by remember { mutableStateOf(false) }

    var inProgress by remember { mutableStateOf(false) }

    var note: Note? by remember { mutableStateOf(null) }

    var text by remember { mutableStateOf("") }

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
            Text(text = "LocalId ${it.id}")
            Text(text = "RemoteId ${it.remoteId ?: "No Remote Id"}")
            Text(text = "CommitId ${it.commitId}")
            Text(text = "State ${it.state}")

            Button(onClick = {
                if (inProgress) {
                    return@Button
                }

                inProgress = true

                coroutineScope.launch {
                    try {
                        NoteViewModel()
                            .deleteNote(it.id)

                        updateNote = false

                        navController.navigateUp()
                    } catch (e: ReaxException) {
                        e.handle()
                    }
                }
            }) {
                Text("Delete Note")
            }
        }

        TextField(value = text, onValueChange = {
            updateNote = true

            text = it
        })
    }

    DisposableEffect(lifecycleOwner) {
        val observer = LifecycleEventObserver { _, event ->
            when (event) {
                Lifecycle.Event.ON_STOP -> GlobalScope.launch {
                    if (updateNote) {
                        note?.let {
                            try {
                                NoteViewModel().updateNote(it.id, text)
                            } catch (e: ReaxException) {
                                e.handle()
                            }
                        }
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
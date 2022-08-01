package com.bwqr.mavinote.ui.note

import android.util.Log
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material.Text
import androidx.compose.material.TextField
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalLifecycleOwner
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.lifecycle.Lifecycle
import androidx.lifecycle.LifecycleEventObserver
import com.bwqr.mavinote.models.ReaxException
import com.bwqr.mavinote.ui.Title
import com.bwqr.mavinote.ui.theme.MaviNoteTheme
import com.bwqr.mavinote.viewmodels.NoteViewModel
import kotlinx.coroutines.GlobalScope
import kotlinx.coroutines.launch

/*
 * Between folderId and noteId, exactly one of them must be not null
 * When folderId is not null and noteId is null, it is meant to create a new note in the given folderId.
 * When noteId is not null and folderId is null, it is meant to update given noteId
 */
@Composable
fun Note(folderId: Int?, noteId: Int?) {
    val lifecycleOwner = LocalLifecycleOwner.current

    var title by remember { mutableStateOf<String?>(null) }
    var text by remember { mutableStateOf("") }
    var modified by remember { mutableStateOf(false) }

    LaunchedEffect(key1 = 1) {
        noteId?.let {
            try {
                val note = NoteViewModel().note(noteId)

                if (note != null) {
                    title = note.title
                    text = note.text
                } else {
                    Log.e("Note", "noteId $noteId does not exist")
                }
            } catch (e: ReaxException) {
                e.handle()
            }
        }
    }

    DisposableEffect(lifecycleOwner) {
        val observer = LifecycleEventObserver { _, event ->
            when (event) {
                Lifecycle.Event.ON_STOP -> GlobalScope.launch {
                    try {
                        if (noteId != null && modified) {
                            NoteViewModel().updateNote(noteId, text)
                        } else if (noteId == null && text.isNotBlank()) {
                            // if noteId is null, folderId must be provided
                            NoteViewModel().createNote(folderId!!, text)
                        }
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
    NoteView(title, text, {
        text = it
        modified = true
    }, {})
}

@Composable
fun NoteView(
    title: String?,
    text: String,
    onTextChange: (text: String) -> Unit,
    onDelete: () -> Unit
) {
    Column(modifier = Modifier.padding(12.dp)) {
        Title(title ?: "New Note")
        Text(text = "Note", modifier = Modifier.padding(0.dp, 0.dp, 0.dp, 12.dp))

        TextField(
            value = text,
            onValueChange = onTextChange,
            placeholder = {
                Text(text = "You can write your note here")
            },
            modifier = Modifier
                .fillMaxHeight()
                .fillMaxWidth()
        )
    }
}

@Preview(showBackground = true)
@Composable
fun NotePreview() {
    MaviNoteTheme {
        NoteView(
            "Shining Note",
            "Here is a little bit description about note",
            {},
            {}
        )
    }
}
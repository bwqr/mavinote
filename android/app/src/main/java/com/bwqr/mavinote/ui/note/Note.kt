package com.bwqr.mavinote.ui.note

import android.util.Log
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.MoreVert
import androidx.compose.material3.AlertDialog
import androidx.compose.material3.Button
import androidx.compose.material3.ButtonDefaults
import androidx.compose.material3.DropdownMenu
import androidx.compose.material3.DropdownMenuItem
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.material3.TextField
import androidx.compose.runtime.Composable
import androidx.compose.runtime.DisposableEffect
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalLifecycleOwner
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.lifecycle.Lifecycle
import androidx.lifecycle.LifecycleEventObserver
import androidx.navigation.NavController
import com.bwqr.mavinote.Bus
import com.bwqr.mavinote.BusEvent
import com.bwqr.mavinote.models.NoteError
import com.bwqr.mavinote.ui.Title
import com.bwqr.mavinote.ui.theme.MavinoteTheme
import com.bwqr.mavinote.viewmodels.NoteViewModel
import kotlinx.coroutines.GlobalScope
import kotlinx.coroutines.launch

/*
 * Between folderId and noteId, exactly one of them must be not null
 * When folderId is not null and noteId is null, it is meant to create a new note in the given folderId.
 * When noteId is not null and folderId is null, it is meant to update given noteId
 */
@Composable
fun Note(navController: NavController, folderId: Int?, noteId: Int?) {
    val lifecycleOwner = LocalLifecycleOwner.current
    val coroutineScope = rememberCoroutineScope()
    var deleting by remember { mutableStateOf(false) }

    var title by remember { mutableStateOf<String?>(null) }
    var text by remember { mutableStateOf("") }
    var modified by remember { mutableStateOf(false) }

    LaunchedEffect(key1 = 1) {
        noteId?.let {
            try {
                val note = NoteViewModel.note(noteId)

                if (note != null) {
                    title = note.name
                    text = note.text
                } else {
                    Log.e("Note", "noteId $noteId does not exist")
                }
            } catch (e: NoteError) {
                e.handle()
            }
        }
    }

    DisposableEffect(lifecycleOwner) {
        val observer = LifecycleEventObserver { _, event ->
            when (event) {
                Lifecycle.Event.ON_STOP -> GlobalScope.launch {
                    try {
                        if (deleting) {
                            return@launch
                        }

                        if (noteId != null && modified) {
                            NoteViewModel.updateNote(noteId, text)
                        } else if (noteId == null && text.isNotBlank()) {
                            // if noteId is null, folderId must be provided
                            NoteViewModel.createNote(folderId!!, text)
                        }
                    } catch (e: NoteError) {
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
    NoteView(title, text,
        onTextChange = {
            text = it
            modified = true
        },
        onDelete = {
            if (deleting) {
                return@NoteView
            }

            deleting = true

            val deletingNoteId = if (noteId != null) {
                noteId
            } else {
                navController.navigateUp()
                return@NoteView
            }

            coroutineScope.launch {
                try {
                    NoteViewModel.deleteNote(deletingNoteId)

                    Bus.emit(BusEvent.ShowMessage("Note is deleted"))
                    navController.navigateUp()
                } catch (e: NoteError) {
                    e.handle()
                    deleting = false
                }
            }
        })
}

@Composable
fun NoteView(
    title: String?,
    text: String,
    onTextChange: (text: String) -> Unit,
    onDelete: () -> Unit
) {
    var expanded by remember { mutableStateOf(false) }
    var showDeleteWarn by remember { mutableStateOf(false) }

    Column(
        verticalArrangement = Arrangement.spacedBy(24.dp),
        modifier = Modifier.padding(16.dp),
    ) {
        Column {
            Row(
                verticalAlignment = Alignment.CenterVertically,
                modifier = Modifier.fillMaxWidth()
            ) {
                Title(title ?: "New Note", modifier = Modifier.weight(1f))
                IconButton(onClick = { expanded = true }) {
                    Icon(imageVector = Icons.Filled.MoreVert, contentDescription = null)
                    DropdownMenu(expanded, onDismissRequest = { expanded = false }) {
                        DropdownMenuItem(
                            onClick = { showDeleteWarn = true },
                            text = { Text(text = "Delete") }
                        )
                    }
                }
            }
            Text(text = "Note")
        }

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

    if (showDeleteWarn) {
        AlertDialog(
            onDismissRequest = { showDeleteWarn = false },
            text = { Text("Are you sure about deleting the note?") },
            confirmButton = {
                Button(
                    modifier = Modifier.fillMaxWidth(),
                    colors = ButtonDefaults.buttonColors(containerColor = MaterialTheme.colorScheme.error),
                    onClick = {
                        showDeleteWarn = false
                        onDelete()
                    },
                ) {
                    Text("Delete Note")
                }
            }
        )
    }
}

@Preview(showBackground = true)
@Composable
fun NotePreview() {
    MavinoteTheme {
        NoteView(
            "Shining Note",
            "Here is a little bit description about note",
            {},
            {}
        )
    }
}
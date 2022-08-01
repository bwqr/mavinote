package com.bwqr.mavinote.ui.note

import android.util.Log
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Add
import androidx.compose.material.icons.filled.MoreVert
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.navigation.NavController
import androidx.navigation.compose.rememberNavController
import com.bwqr.mavinote.models.Folder
import com.bwqr.mavinote.models.Note
import com.bwqr.mavinote.models.ReaxException
import com.bwqr.mavinote.models.State
import com.bwqr.mavinote.ui.Title
import com.bwqr.mavinote.ui.theme.MaviNoteTheme
import com.bwqr.mavinote.viewmodels.NoteViewModel
import kotlinx.coroutines.flow.catch
import kotlinx.coroutines.flow.launchIn
import kotlinx.coroutines.flow.onEach
import kotlinx.coroutines.launch

@Composable
fun Notes(navController: NavController, folderId: Int) {
    val coroutineScope = rememberCoroutineScope()
    var deleting by remember { mutableStateOf(false) }

    var folder by remember { mutableStateOf<Folder?>(null) }

    var notes by remember { mutableStateOf(listOf<Note>()) }

    LaunchedEffect(key1 = folderId) {
        launch {
            try {
                folder = NoteViewModel().folder(folderId)
                if (folder == null) {
                    Log.e("Notes", "folderId $folderId does not exist")
                }
            } catch (e: ReaxException) {
                e.handle()
            }
        }

        NoteViewModel()
            .notes(folderId)
            .onEach { notes = it }
            .catch {
                when (val cause = it.cause) {
                    is ReaxException -> cause.handle()
                }
            }
            .launchIn(this)
    }

    folder?.let {
        NotesView(navController, it, notes) {
            if (deleting) {
                return@NotesView
            }

            deleting = true

            coroutineScope.launch {
                try {
                    NoteViewModel().deleteFolder(folderId)

                    navController.navigateUp()
                } catch (e: ReaxException) {
                    e.handle()
                } finally {
                    deleting = false
                }
            }
        }
    }
}

@Composable
fun NotesView(
    navController: NavController,
    folder: Folder,
    notes: List<Note>,
    onDelete: () -> Unit
) {
    var expanded by remember { mutableStateOf(false) }

    Column(modifier = Modifier.padding(12.dp)) {
        Row(verticalAlignment = Alignment.CenterVertically, modifier = Modifier.fillMaxWidth()) {
            Title(folder.name, modifier = Modifier.weight(1f))
            IconButton(onClick = { expanded = true }) {
                Icon(imageVector = Icons.Filled.MoreVert, contentDescription = null)
                DropdownMenu(expanded, onDismissRequest = { expanded = false }) {
                    DropdownMenuItem(onClick = onDelete) {
                        Text(text = "Delete")
                    }
                }
            }
        }

        Text(text = "Folder", modifier = Modifier.padding(0.dp, 0.dp, 0.dp, 12.dp))

        if (notes.isEmpty()) {
            Text(text = "There is no note in this folder")
        } else {
            Card(
                elevation = 1.dp,
                modifier = Modifier
                    .padding(24.dp, 0.dp, 0.dp, 0.dp)
                    .fillMaxWidth()
                    .padding(0.dp, 0.dp, 0.dp, 18.dp)
            ) {
                LazyColumn {
                    items(notes) { note ->
                        Text(
                            note.title ?: "New Note",
                            modifier = Modifier
                                .fillMaxWidth()
                                .clickable { navController.navigate("note?noteId=${note.id}") }
                                .padding(16.dp, 12.dp)
                        )
                    }
                }
            }
        }

    }
}

@Composable
fun NotesFab(navController: NavController, folderId: Int) {
    FloatingActionButton(onClick = { navController.navigate("note?folderId=$folderId") }) {
        Icon(Icons.Filled.Add, contentDescription = null)
    }
}

@Preview(showBackground = true)
@Composable
fun NotesPreview() {
    val navController = rememberNavController()

    val folder = Folder(1, 1, null, "Can Long Typed Title Fit Here Or Cannot Fit Here", State.Clean)

    val notes = listOf(
        Note(1, folder.id, null, "Downtown", "Going to downtown", 1, State.Clean),
        Note(2, folder.id, null, "Hometown", "Sky hometown", 1, State.Clean),
        Note(3, folder.id, null, "Middle Town", "Right in the middle", 1, State.Clean),
        Note(4, folder.id, null, "Middle ", "Right in the middle", 1, State.Clean),
    )

    MaviNoteTheme {
        NotesView(navController, folder, notes) {}
    }
}

@Preview(showBackground = true)
@Composable
fun EmptyNotesPreview() {
    val navController = rememberNavController()

    val folder = Folder(1, 1, null, "Todos", State.Clean)

    MaviNoteTheme {
        NotesView(navController, folder, listOf()) {}
    }
}
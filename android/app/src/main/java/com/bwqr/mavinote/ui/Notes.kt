package com.bwqr.mavinote.ui

import androidx.compose.foundation.clickable
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material.Text
import androidx.compose.foundation.lazy.items
import androidx.compose.runtime.Composable
import androidx.compose.runtime.remember
import androidx.compose.runtime.*
import com.bwqr.mavinote.viewmodels.NoteViewModel
import androidx.compose.material.FloatingActionButton
import androidx.compose.material.Icon
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Add
import androidx.compose.runtime.mutableStateOf
import androidx.compose.ui.Modifier
import androidx.navigation.NavController
import com.bwqr.mavinote.models.Note
import kotlinx.coroutines.launch

@Composable
fun Notes(navController: NavController, folderId: Int) {
    var notes by remember {
        mutableStateOf(listOf<Note>())
    }

    LaunchedEffect(key1 = folderId) {
        notes = NoteViewModel().notes(folderId).getOrThrow()
    }

    LazyColumn {
        items(notes) { note ->
            Text(text = note.title, Modifier.clickable {
                navController.navigate("note/${note.id}")
            })
        }
    }
}

@Composable
fun NotesFab(navController: NavController, folderId: Int) {
    val scope = rememberCoroutineScope()

    FloatingActionButton(onClick = {
        scope.launch {
            val addedNoteId = NoteViewModel().createNote(folderId).getOrThrow()

            navController.navigate("note/$addedNoteId")
        }
    }) {
        Icon(Icons.Filled.Add, contentDescription = null)
    }
}
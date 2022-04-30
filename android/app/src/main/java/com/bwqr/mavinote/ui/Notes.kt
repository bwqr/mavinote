package com.bwqr.mavinote.ui

import androidx.compose.foundation.clickable
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.remember
import com.bwqr.mavinote.viewmodels.NoteViewModel
import androidx.compose.foundation.lazy.items
import androidx.compose.material.FloatingActionButton
import androidx.compose.material.Icon
import androidx.compose.material.Scaffold
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Add
import androidx.compose.ui.Modifier
import androidx.navigation.NavController

@Composable
fun Notes(navController: NavController, folderId: Int) {
    val notes = remember {
        NoteViewModel().notes(folderId)
    }

    Scaffold(
        floatingActionButton = {
            FloatingActionButton(onClick = {
                val addedNoteId = NoteViewModel().createNote(folderId)

                navController.navigate("note/$addedNoteId")
            }) {
                Icon(Icons.Filled.Add, contentDescription = null)
            }
        }
    ) {
        LazyColumn {
            items(notes) { note ->
                Text(text = note.title, Modifier.clickable {
                    navController.navigate("note/${note.id}")
                })
            }
        }
    }
}
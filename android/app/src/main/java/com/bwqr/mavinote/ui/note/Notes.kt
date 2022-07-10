package com.bwqr.mavinote.ui.note

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material.Button
import androidx.compose.material.FloatingActionButton
import androidx.compose.material.Icon
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Add
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontStyle
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.TextUnit
import androidx.compose.ui.unit.TextUnitType
import androidx.navigation.NavController
import com.bwqr.mavinote.models.Folder
import com.bwqr.mavinote.models.Note
import com.bwqr.mavinote.models.ReaxException
import com.bwqr.mavinote.ui.NoteScreens
import com.bwqr.mavinote.viewmodels.NoteViewModel
import kotlinx.coroutines.flow.catch
import kotlinx.coroutines.flow.launchIn
import kotlinx.coroutines.flow.onEach
import kotlinx.coroutines.launch

@Composable
fun Notes(navController: NavController, folderId: Int) {
    val coroutineScope = rememberCoroutineScope()

    var folder by remember {
        mutableStateOf<Folder?>(null)
    }
    var notes by remember {
        mutableStateOf(listOf<Note>())
    }

    var inProgress by remember {
        mutableStateOf(false)
    }

    LaunchedEffect(key1 = folderId) {
        try {
            folder = NoteViewModel().folder(folderId)
        } catch(e: ReaxException) {
            e.handle()
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

    Column {
        folder?.let {
            Text(it.name, fontWeight = FontWeight.Bold)
        }

        Button(onClick = {
            if (inProgress) {
                return@Button
            }

            inProgress = true

            coroutineScope.launch {
                try {
                    NoteViewModel().deleteFolder(folderId)

                    navController.navigate(NoteScreens.Folders.route)
                } catch (e: ReaxException) {
                    e.handle()
                } finally {
                    inProgress = false
                }
            }
        }) {
            Text(text = "Delete folder")
        }
        LazyColumn {
            items(notes) { note ->
                Text(text = note.title ?: "New Note", Modifier.clickable {
                    navController.navigate("note/${note.id}")
                })
            }
        }
    }

}

@Composable
fun NotesFab(navController: NavController, folderId: Int) {
    val scope = rememberCoroutineScope()

    FloatingActionButton(onClick = {
        scope.launch {
            val addedNoteId = NoteViewModel().createNote(folderId)

            navController.navigate("note/$addedNoteId")
        }
    }) {
        Icon(Icons.Filled.Add, contentDescription = null)
    }
}
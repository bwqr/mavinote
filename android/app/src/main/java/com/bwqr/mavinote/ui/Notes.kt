package com.bwqr.mavinote.ui

import androidx.compose.foundation.clickable
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.remember
import com.bwqr.mavinote.viewmodels.NoteViewModel
import androidx.compose.foundation.lazy.items
import androidx.compose.ui.Modifier
import androidx.navigation.NavController
import com.bwqr.mavinote.Screen

@Composable
fun Notes(navController: NavController, folderId: Int) {
    val notes = remember {
        NoteViewModel().notes(folderId)
    }
    
    LazyColumn {
        items(notes) { note ->
            Text(text = note.title, Modifier.clickable {
                navController.navigate("note/${note.id}")
            })
        }
    }
}
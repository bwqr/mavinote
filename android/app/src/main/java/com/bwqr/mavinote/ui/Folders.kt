package com.bwqr.mavinote.ui

import androidx.compose.foundation.clickable
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material.FloatingActionButton
import androidx.compose.material.Icon
import androidx.compose.material.Scaffold
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Add
import androidx.compose.runtime.Composable
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.navigation.NavController
import com.bwqr.mavinote.Screen
import com.bwqr.mavinote.viewmodels.NoteViewModel

@Composable
fun Folders(navController: NavController) {
    val folders = remember {
        NoteViewModel().folders()
    }

    Scaffold(
        floatingActionButton = {
            FloatingActionButton(onClick = { navController.navigate(Screen.FolderAdd.route) }) {
                Icon(Icons.Filled.Add, contentDescription = null)
            }
        }
    ) {
        LazyColumn {
            items(folders) { folder ->
                Text(folder.name, Modifier.clickable {
                    navController.navigate("notes/${folder.id}")
                })
            }
        }
    }
}
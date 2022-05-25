package com.bwqr.mavinote.ui.note

import androidx.compose.foundation.clickable
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material.FloatingActionButton
import androidx.compose.material.Icon
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Add
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.navigation.NavController
import com.bwqr.mavinote.Screen
import com.bwqr.mavinote.models.Folder
import com.bwqr.mavinote.models.ReaxException
import com.bwqr.mavinote.viewmodels.NoteViewModel
import kotlinx.coroutines.flow.catch
import kotlinx.coroutines.flow.launchIn
import kotlinx.coroutines.flow.onEach

@Composable
fun Folders(navController: NavController) {
    var folders by remember {
        mutableStateOf(listOf<Folder>())
    }

    LaunchedEffect(key1 = 1) {
        NoteViewModel()
            .folders()
            .onEach { folders = it }
            .catch {
                when (val cause = it.cause) {
                    is ReaxException -> cause.handle()
                }
            }
            .launchIn(this)
    }

    LazyColumn {
        items(folders) { folder ->
            Text(folder.name, Modifier.clickable {
                navController.navigate("notes/${folder.id}")
            })
        }
    }
}

@Composable
fun FolderFab(navController: NavController) {
    FloatingActionButton(onClick = { navController.navigate(Screen.FolderAdd.route) }) {
        Icon(Icons.Filled.Add, contentDescription = null)
    }
}
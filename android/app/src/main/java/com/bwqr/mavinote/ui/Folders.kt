package com.bwqr.mavinote.ui

import androidx.compose.foundation.clickable
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Add
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.navigation.NavController
import com.bwqr.mavinote.Screen
import com.bwqr.mavinote.models.Folder
import com.bwqr.mavinote.models.ReaxException
import com.bwqr.mavinote.viewmodels.Bus
import com.bwqr.mavinote.viewmodels.BusEvent
import com.bwqr.mavinote.viewmodels.NoteViewModel

@Composable
fun Folders(navController: NavController) {
    var folders by remember {
        mutableStateOf(listOf<Folder>())
    }

    LaunchedEffect(key1 = 1) {
        try {
            folders = NoteViewModel().folders().getOrThrow()
        } catch (e: ReaxException) {
            Bus.emitter().emit(BusEvent.NoInternetConnection)
        }
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
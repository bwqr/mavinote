package com.bwqr.mavinote.ui.note

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.material.Button
import androidx.compose.material.OutlinedTextField
import androidx.compose.material.Text
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.navigation.NavController
import com.bwqr.mavinote.Screen
import com.bwqr.mavinote.models.ReaxException
import com.bwqr.mavinote.viewmodels.NoteViewModel
import kotlinx.coroutines.launch

@Composable
fun FolderAdd(navController: NavController) {
    val scope = rememberCoroutineScope()

    var name by remember {
        mutableStateOf("")
    }

    Column(
        modifier = Modifier
            .fillMaxWidth(),
        horizontalAlignment = Alignment.CenterHorizontally
    ) {
        OutlinedTextField(
            value = name,
            onValueChange = { name = it },
            label = { Text("Name") }
        )

        Button(
            modifier = Modifier.fillMaxWidth(),
            onClick = {
                scope.launch {
                    try {
                        NoteViewModel().addFolder(name)
                    } catch (e: ReaxException) {
                        e.handle()
                    }

                    navController.navigate(Screen.Folders.route)
                }
            }
        ) {
            Text("Add")
        }
    }
}
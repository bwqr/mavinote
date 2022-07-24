package com.bwqr.mavinote.ui.note

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material.Button
import androidx.compose.material.OutlinedTextField
import androidx.compose.material.Text
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.navigation.NavController
import com.bwqr.mavinote.models.Account
import com.bwqr.mavinote.models.ReaxException
import com.bwqr.mavinote.viewmodels.NoteViewModel
import com.bwqr.mavinote.ui.NoteScreens
import kotlinx.coroutines.flow.catch
import kotlinx.coroutines.flow.launchIn
import kotlinx.coroutines.flow.onEach
import kotlinx.coroutines.launch

@Composable
fun FolderAdd(navController: NavController) {
    val scope = rememberCoroutineScope()
    var inProgress by remember { mutableStateOf(false) }

    var error by remember { mutableStateOf("") }

    var accounts by remember { mutableStateOf(listOf<Account>()) }

    var name by remember { mutableStateOf("") }

    var accountId by remember { mutableStateOf<Int?>(null) }

    LaunchedEffect(key1 = 0) {
        NoteViewModel()
            .accounts()
            .onEach { accounts = it }
            .catch {
                when (val cause = it.cause) {
                    is ReaxException -> cause.handle()
                }
            }
            .launchIn(this)
    }

    Column(
        modifier = Modifier
            .fillMaxWidth(),
        horizontalAlignment = Alignment.CenterHorizontally
    ) {
        LazyColumn {
            items(accounts) { account ->
                Text(
                    text = account.kind.toString(),
                    fontWeight = if (accountId == account.id) FontWeight.Bold else FontWeight.Normal,
                    modifier = Modifier.clickable { accountId = account.id }
                )
            }
        }

        OutlinedTextField(
            value = name,
            onValueChange = { name = it },
            label = { Text("Name") }
        )

        if (error.isNotEmpty()) {
            Text(text = error)
        }

        Button(
            modifier = Modifier.fillMaxWidth(),
            onClick = {
                if (inProgress) {
                    return@Button
                }

                if (name.isEmpty()) {
                    error = "Please give a name to folder"
                    return@Button
                }

                if (accountId == null) {
                    error = "Please select an account"
                }
                val accountId = accountId ?: return@Button

                inProgress = true
                error = ""

                scope.launch {
                    try {
                        NoteViewModel().addFolder(accountId, name)

                        navController.navigate(NoteScreens.Folders.route)
                    } catch (e: ReaxException) {
                        e.handle()
                    } finally {
                        inProgress = false
                    }
                }
            }
        ) {
            Text("Add")
        }
    }
}
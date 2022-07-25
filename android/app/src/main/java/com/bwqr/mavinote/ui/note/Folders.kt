package com.bwqr.mavinote.ui.note

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material.FloatingActionButton
import androidx.compose.material.Icon
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Add
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.ExperimentalUnitApi
import androidx.compose.ui.unit.TextUnit
import androidx.compose.ui.unit.TextUnitType
import androidx.navigation.NavController
import com.bwqr.mavinote.models.Account
import com.bwqr.mavinote.models.Folder
import com.bwqr.mavinote.models.ReaxException
import com.bwqr.mavinote.viewmodels.NoteViewModel
import com.bwqr.mavinote.ui.NoteScreens
import kotlinx.coroutines.flow.catch
import kotlinx.coroutines.flow.launchIn
import kotlinx.coroutines.flow.onEach
import kotlinx.coroutines.flow.combine

data class AccountWithFolders(
    val account: Account,
    val folders: List<Folder>,
)

@OptIn(ExperimentalUnitApi::class)
@Composable
fun Folders(navController: NavController) {
    var accounts by remember {
        mutableStateOf(listOf<AccountWithFolders>())
    }

    LaunchedEffect(key1 = 1) {
        NoteViewModel()
            .accounts()
            .combine(NoteViewModel().folders()) { accounts, folders ->
                accounts.map {
                    AccountWithFolders(
                        it,
                        folders.filter { folder -> folder.accountId == it.id })
                }
            }
            .onEach { accounts = it }
            .catch {
                when (val cause = it.cause) {
                    is ReaxException -> cause.handle()
                }
            }
            .launchIn(this)
    }

    Column {
        Text(
            text = "Folders",
            fontWeight = FontWeight.ExtraBold,
            fontSize = TextUnit(6f, TextUnitType.Em)
        )

        for (account in accounts) {
            Text(
                text = account.account.kind.toString(),
                fontWeight = FontWeight.Bold,
                fontSize = TextUnit(4.5f, TextUnitType.Em)
            )

            LazyColumn {
                items(account.folders) { folder ->
                    Text(folder.name, Modifier.clickable {
                        navController.navigate("notes/${folder.id}")
                    })
                }
            }
        }
    }
}

@Composable
fun FolderFab(navController: NavController) {
    FloatingActionButton(onClick = { navController.navigate(NoteScreens.FolderAdd.route) }) {
        Icon(Icons.Filled.Add, contentDescription = null)
    }
}
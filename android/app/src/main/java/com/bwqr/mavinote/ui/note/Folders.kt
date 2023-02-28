package com.bwqr.mavinote.ui.note

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Add
import androidx.compose.material.icons.filled.MoreVert
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.navigation.NavController
import androidx.navigation.compose.rememberNavController
import com.bwqr.mavinote.R
import com.bwqr.mavinote.models.*
import com.bwqr.mavinote.models.State
import com.bwqr.mavinote.ui.Screen
import com.bwqr.mavinote.ui.Title
import com.bwqr.mavinote.ui.theme.MavinoteTheme
import com.bwqr.mavinote.ui.theme.Typography
import com.bwqr.mavinote.viewmodels.AccountViewModel
import com.bwqr.mavinote.viewmodels.NoteViewModel
import kotlinx.coroutines.flow.catch
import kotlinx.coroutines.flow.combine
import kotlinx.coroutines.flow.launchIn
import kotlinx.coroutines.flow.onEach

data class AccountWithFolders(
    val account: Account,
    val folders: List<Folder>,
)

@Composable
fun Folders(navController: NavController) {
    var accounts by remember {
        mutableStateOf(listOf<AccountWithFolders>())
    }

    LaunchedEffect(key1 = 1) {
        AccountViewModel
            .accounts()
            .combine(NoteViewModel.folders()) { accounts, folders ->
                accounts.map {
                    AccountWithFolders(
                        it,
                        folders.filter { folder -> folder.accountId == it.id })
                }
            }
            .onEach { accounts = it }
            .catch {
                when (val cause = it.cause) {
                    is NoteError -> cause.handle()
                }
            }
            .launchIn(this)
    }

    FoldersView(navController, accounts)
}

@Composable
fun FoldersView(
    navController: NavController,
    accounts: List<AccountWithFolders>,
) {
    var expanded by remember { mutableStateOf(false) }

    Column(modifier = Modifier.padding(12.dp)) {
        Row(verticalAlignment = Alignment.CenterVertically, modifier = Modifier.fillMaxWidth()) {
            Title(
                stringResource(R.string.folders),
                modifier = Modifier
                    .padding(0.dp, 0.dp, 0.dp, 12.dp)
                    .weight(1f)
            )
            IconButton(onClick = { expanded = true }) {
                Icon(imageVector = Icons.Filled.MoreVert, contentDescription = null)
                DropdownMenu(expanded, onDismissRequest = { expanded = false }) {
                    DropdownMenuItem(
                        onClick = { navController.navigate(Screen.Account.Accounts.route) },
                        text = { Text(text = stringResource(R.string.manage_accounts)) }
                    )
                }
            }
        }


        for (account in accounts) {
            Row(
                verticalAlignment = Alignment.Bottom,
                modifier = Modifier
                    .fillMaxWidth()
                    .padding(0.dp, 0.dp, 0.dp, 6.dp)
            ) {
                Text(
                    account.account.name,
                    style = Typography.titleSmall,
                    modifier = Modifier
                        .padding(24.dp + 16.dp, 0.dp, 0.dp, 0.dp)
                )

                Text(
                    text = account.folders.size.toString(),
                    textAlign = TextAlign.End,
                    modifier = Modifier.fillMaxWidth()
                )
            }

            if (account.folders.isEmpty()) {
                Text(
                    text = stringResource(R.string.no_folder_in_account),
                    modifier = Modifier.padding(24.dp + 16.dp, 12.dp, 24.dp + 16.dp, 18.dp + 12.dp)
                )
            } else {
                Card(
                    modifier = Modifier
                        .padding(24.dp, 0.dp, 0.dp, 0.dp)
                        .fillMaxWidth()
                        .padding(0.dp, 0.dp, 0.dp, 18.dp)
                ) {
                    LazyColumn {
                        items(account.folders) { folder ->
                            Text(
                                folder.name,
                                modifier = Modifier
                                    .fillMaxWidth()
                                    .clickable { navController.navigate("notes/${folder.id}") }
                                    .padding(16.dp, 12.dp)
                            )
                        }
                    }
                }
            }
        }
    }
}

@Composable
fun FoldersFab(navController: NavController) {
    ExtendedFloatingActionButton(
        text = { Text("Folder") },
        icon = { Icon(Icons.Filled.Add, contentDescription = null) },
        onClick = { navController.navigate(Screen.Note.FolderCreate.route) }
    )
}

@Preview(showBackground = true)
@Composable
fun FoldersPreview() {
    val navController = rememberNavController()

    val accounts = listOf(
        AccountWithFolders(
            Account(1, "Default", AccountKind.Local),
            listOf(
                Folder(1, 1, null, "Favorites", State.Clean),
                Folder(2, 1, null, "Todos", State.Clean)
            )
        ),
        AccountWithFolders(
            Account(2, "Remote", AccountKind.Mavinote),
            listOf(Folder(1, 2, null, "Race Cars", State.Clean))
        )
    )

    MavinoteTheme {
        FoldersView(navController, accounts)
    }
}
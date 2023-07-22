package com.bwqr.mavinote.ui.note

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Add
import androidx.compose.material.icons.filled.KeyboardArrowRight
import androidx.compose.material.icons.outlined.Settings
import androidx.compose.material3.Card
import androidx.compose.material3.ExtendedFloatingActionButton
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.navigation.NavController
import androidx.navigation.compose.rememberNavController
import com.bwqr.mavinote.R
import com.bwqr.mavinote.models.Account
import com.bwqr.mavinote.models.AccountKind
import com.bwqr.mavinote.models.Folder
import com.bwqr.mavinote.models.NoteError
import com.bwqr.mavinote.models.State
import com.bwqr.mavinote.ui.Screen
import com.bwqr.mavinote.ui.theme.MavinoteTheme
import com.bwqr.mavinote.ui.theme.Spacing
import com.bwqr.mavinote.ui.theme.Typography
import com.bwqr.mavinote.ui.util.Title
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
    Column(verticalArrangement = Arrangement.spacedBy(Spacing.SectionSpacing)) {
        Row(
            verticalAlignment = Alignment.CenterVertically,
            modifier = Modifier
                .fillMaxWidth()
                .padding(
                    top = Spacing.ScreenPadding,
                    start = Spacing.ScreenPadding,
                    end = Spacing.ScreenPadding
                )
        ) {
            Title(
                stringResource(R.string.folders),
                modifier = Modifier
                    .weight(1f)
            )
            IconButton(onClick = { navController.navigate(Screen.Misc.Navigations.route) }) {
                Icon(imageVector = Icons.Outlined.Settings, contentDescription = null)
            }
        }

        LazyColumn(verticalArrangement = Arrangement.spacedBy(16.dp)) {
            items(accounts) { account ->
                Column(
                    verticalArrangement = Arrangement.spacedBy(16.dp),
                    modifier = Modifier.padding(horizontal = Spacing.ScreenPadding)
                ) {
                    Row(
                        verticalAlignment = Alignment.Bottom,
                        modifier = Modifier
                            .fillMaxWidth()
                    ) {
                        Text(
                            account.account.name,
                            style = Typography.titleMedium,
                            color = Color.Gray,
                            modifier = Modifier
                                .padding(start = 24.dp)
                        )

                        Text(
                            text = account.folders.size.toString(),
                            textAlign = TextAlign.End,
                            style = Typography.titleMedium,
                            color = Color.Gray,
                            modifier = Modifier
                                .fillMaxWidth()
                                .padding(end = 24.dp)
                        )
                    }

                    if (account.folders.isEmpty()) {
                        Text(
                            text = stringResource(R.string.no_folder_in_account),
                            modifier = Modifier.padding(16.dp, 8.dp)
                        )
                    } else {
                        Card(modifier = Modifier.fillMaxWidth()) {
                            for (folder in account.folders) {
                                Row(
                                    verticalAlignment = Alignment.CenterVertically,
                                    modifier = Modifier.clickable { navController.navigate("notes/${folder.id}") }
                                ) {
                                    Text(
                                        folder.name,
                                        modifier = Modifier
                                            .weight(1f)
                                            .padding(24.dp, 12.dp)
                                    )

                                    Icon(
                                        Icons.Filled.KeyboardArrowRight,
                                        contentDescription = null,
                                        tint = Color.Gray,
                                        modifier = Modifier.padding(16.dp)
                                    )
                                }
                            }
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
                Folder(2, 1, null, "Todos", State.Clean),
                Folder(3, 1, null, "Hobbies", State.Clean)
            )
        ),
        AccountWithFolders(
            Account(2, "Merhaba", AccountKind.Mavinote),
            listOf()
        ),
        AccountWithFolders(
            Account(2, "Remote", AccountKind.Mavinote),
            listOf(Folder(1, 2, null, "Race Cars", State.Clean))
        ),
    )

    MavinoteTheme {
        FoldersView(navController, accounts)
    }
}
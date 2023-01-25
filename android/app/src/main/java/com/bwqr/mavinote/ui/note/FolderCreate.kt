package com.bwqr.mavinote.ui.note

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.navigation.NavController
import com.bwqr.mavinote.R
import com.bwqr.mavinote.models.Account
import com.bwqr.mavinote.models.AccountKind
import com.bwqr.mavinote.models.NoteError
import com.bwqr.mavinote.ui.Title
import com.bwqr.mavinote.viewmodels.NoteViewModel
import kotlinx.coroutines.flow.catch
import kotlinx.coroutines.flow.launchIn
import kotlinx.coroutines.flow.onEach
import kotlinx.coroutines.launch

@Composable
fun FolderCreate(navController: NavController) {
    val coroutineScope = rememberCoroutineScope()
    var inProgress by remember { mutableStateOf(false) }

    var accounts by remember { mutableStateOf<List<Account>?>(null) }
    var error by remember { mutableStateOf<String?>(null) }

    LaunchedEffect(key1 = 0) {
        NoteViewModel
            .accounts()
            .onEach { accounts = it }
            .catch {
                when (val cause = it.cause) {
                    is NoteError -> cause.handle()
                }
            }
            .launchIn(this)
    }

    accounts?.let {
        FolderCreateView(it, error) { accountId, folderName ->
            if (inProgress) {
                return@FolderCreateView
            }

            if (accountId == null || folderName.isBlank()) {
                error = "Please specify a folder name and select an account"
                return@FolderCreateView
            }

            error = ""
            inProgress = true

            coroutineScope.launch {
                try {
                    NoteViewModel.createFolder(accountId, folderName)

                    navController.navigateUp()
                } catch (e: NoteError) {
                    e.handle()
                } finally {
                    inProgress = false
                }
            }
        }
    }
}

@Composable
fun FolderCreateView(
    accounts: List<Account>,
    error: String?,
    onCreateFolder: (accountId: Int?, folderName: String) -> Unit
) {
    var folderName by remember { mutableStateOf("") }
    var accountId by remember { mutableStateOf(accounts.firstOrNull()?.id) }

    Column(modifier = Modifier.padding(12.dp)) {
        Title(
            stringResource(R.string.create_folder),
            modifier = Modifier.padding(0.dp, 0.dp, 0.dp, 32.dp)
        )

        Column(
            modifier = Modifier
                .padding(40.dp, 0.dp, 0.dp, 0.dp)
        ) {
            Column(modifier = Modifier.padding(0.dp, 0.dp, 0.dp, 32.dp)) {
                Text(
                    text = stringResource(R.string.folder_name),
                    modifier = Modifier.padding(0.dp, 0.dp, 0.dp, 12.dp)
                )
                TextField(value = folderName, onValueChange = { folderName = it })
            }

            Column(modifier = Modifier.padding(0.dp, 0.dp, 0.dp, 32.dp)) {
                Text(
                    text = stringResource(R.string.account_this_folder_will_be_created),
                    modifier = Modifier.padding(0.dp, 0.dp, 0.dp, 12.dp)
                )
                LazyColumn {
                    items(accounts) { account ->
                        Row(verticalAlignment = Alignment.CenterVertically) {
                            RadioButton(
                                selected = accountId == account.id,
                                onClick = { accountId = account.id }
                            )
                            Text(account.name)
                        }
                    }
                }
            }

            if (error != null) {
                Text(
                    text = error,
                    color = MaterialTheme.colors.error,
                    modifier = Modifier.padding(0.dp, 0.dp, 0.dp, 32.dp)
                )
            }
        }

        Box(contentAlignment = Alignment.BottomCenter, modifier = Modifier.weight(1f)) {
            Button(
                onClick = { onCreateFolder(accountId, folderName) },
                modifier = Modifier.fillMaxWidth()
            ) {
                Text(text = stringResource(id = R.string.create_folder))
            }
        }
    }
}

@Preview(showBackground = true)
@Composable
fun FolderCreatePreview() {
    val accounts = listOf(
        Account(1, "Default", AccountKind.Local),
        Account(2, "Remote", AccountKind.Mavinote)
    )

    val error = null

    FolderCreateView(accounts, error) { _, _ -> }
}
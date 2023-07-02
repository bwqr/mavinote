package com.bwqr.mavinote.ui.note

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.Button
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.RadioButton
import androidx.compose.material3.Text
import androidx.compose.material3.TextField
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.runtime.setValue
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
import com.bwqr.mavinote.viewmodels.AccountViewModel
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
    var validationErrors by remember { mutableStateOf<Set<ValidationErrors>>(setOf()) }

    LaunchedEffect(key1 = 0) {
        AccountViewModel
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
        FolderCreateView(it, validationErrors) { accountId, folderName ->
            if (inProgress) {
                return@FolderCreateView
            }

            val validations = mutableSetOf<ValidationErrors>()

            if (folderName.isBlank()) {
                validations.add(ValidationErrors.InvalidFolderName)
            }

            if (accountId == null) {
                validations.add(ValidationErrors.InvalidAccount)
            }

            validationErrors = validations
            if (validationErrors.isNotEmpty()) {
                return@FolderCreateView
            }

            inProgress = true

            coroutineScope.launch {
                try {
                    NoteViewModel.createFolder(accountId!!, folderName)

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

private enum class ValidationErrors {
    InvalidFolderName,
    InvalidAccount
}

@Composable
private fun FolderCreateView(
    accounts: List<Account>,
    validationErrors: Set<ValidationErrors>,
    onCreateFolder: (accountId: Int?, folderName: String) -> Unit
) {
    val scrollState = rememberScrollState()

    var folderName by remember { mutableStateOf("") }
    var accountId by remember { mutableStateOf(accounts.firstOrNull()?.id) }

    Column(verticalArrangement = Arrangement.spacedBy(24.dp), modifier = Modifier.padding(16.dp)) {
        Title(stringResource(R.string.create_folder))

        Column(
            verticalArrangement = Arrangement.spacedBy(24.dp),
            modifier = Modifier.weight(1f).verticalScroll(scrollState),
        ) {
            Column(verticalArrangement = Arrangement.spacedBy(12.dp)) {
                Text(stringResource(R.string.folder_name))
                TextField(
                    modifier = Modifier.fillMaxWidth(1.0f),
                    value = folderName,
                    onValueChange = { folderName = it },
                )
                if (validationErrors.contains(ValidationErrors.InvalidFolderName)) {
                    Text(text = "Please specify a folder name", color = MaterialTheme.colorScheme.error)
                }
            }

            if (accounts.size > 1) {
                Column {
                    Text(stringResource(R.string.account_this_folder_will_be_created))

                    for (account in accounts) {
                        Row(
                            horizontalArrangement = Arrangement.Start,
                            verticalAlignment = Alignment.CenterVertically,
                            modifier = Modifier.clickable { accountId = account.id }
                        ) {
                            RadioButton(
                                selected = accountId == account.id,
                                onClick = { accountId = account.id }
                            )
                            Text(account.name)
                        }
                    }

                    if (validationErrors.contains(ValidationErrors.InvalidAccount)) {
                        Text(text = "Please specify an account", color = MaterialTheme.colorScheme.error)
                    }
                }
            }
        }

        Button(
            onClick = { onCreateFolder(accountId, folderName) },
            modifier = Modifier.fillMaxWidth()
        ) {
            Text(text = stringResource(id = R.string.create_folder))
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

    val validationErrors = setOf(ValidationErrors.InvalidFolderName, ValidationErrors.InvalidAccount)

    FolderCreateView(accounts, validationErrors) { _, _ -> }
}
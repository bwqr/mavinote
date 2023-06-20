package com.bwqr.mavinote.ui.account

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.KeyboardArrowRight
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.navigation.NavController
import androidx.navigation.compose.rememberNavController
import com.bwqr.mavinote.Bus
import com.bwqr.mavinote.R
import com.bwqr.mavinote.models.*
import com.bwqr.mavinote.ui.Title
import com.bwqr.mavinote.ui.theme.Typography
import com.bwqr.mavinote.viewmodels.AccountViewModel
import kotlinx.coroutines.launch

@Composable
fun Account(navController: NavController, accountId: Int) {
    val scope = rememberCoroutineScope()
    var inProgress by remember { mutableStateOf(false) }
    var account by remember { mutableStateOf<Account?>(null) }
    var mavinote by remember { mutableStateOf<Mavinote?>(null) }

    fun removeAccount() {
        if (inProgress) {
            return
        }

        inProgress = true

        scope.launch {
            try {
                AccountViewModel.removeAccount(accountId)
                Bus.message("Account is removed")
                navController.navigateUp()
            } catch (e: NoteError) {
                when {
                    e is MavinoteError.Message && e.message == "cannot_delete_only_remaining_device" -> {
                        Bus.message("This device is the only remaining device for this account. If you want to close the account, choose Close Account option.")
                    }
                    else -> e.handle()
                }
            } finally {
                inProgress = false
            }
        }
    }

    LaunchedEffect(key1 = 0) {
        try {
            account = AccountViewModel.account(accountId)

            account?.let {
                if (AccountKind.Mavinote == it.kind) {
                    try {
                        mavinote = AccountViewModel.mavinoteAccount(it.id)
                    } catch (e: NoteError) {
                        e.handle()
                    }
                }
            }
        } catch (e: NoteError) {
            e.handle()
        }
    }

    account?.let { it ->
        AccountView(
            navController,
            it,
            mavinote,
        ) { removeAccount() }
    }
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun AccountView(
    navController: NavController,
    account: Account,
    mavinote: Mavinote?,
    onRemoveAccount: () -> Unit,
) {
    Column(modifier = Modifier.padding(12.dp)) {
        Row {
            Title(account.name, modifier = Modifier.weight(1f))
        }

        Text(
            text = stringResource(R.string.account),
            modifier = Modifier.padding(0.dp, 0.dp, 0.dp, 12.dp)
        )

        Column {
            ListItem(
                headlineText = { Text(stringResource(R.string.name)) },
                trailingContent = { Text(account.name, style = Typography.bodyMedium) }
            )

            Divider()

            ListItem(
                headlineText = { Text(stringResource(R.string.kind)) },
                trailingContent = { Text(account.kind.toString(), style = Typography.bodyMedium) },
            )

        }

        mavinote?.let {
            Divider()

            MavinoteAccountView(navController, account.id, it, onRemoveAccount)
        }
    }
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun MavinoteAccountView(
    navController: NavController,
    accountId: Int,
    mavinote: Mavinote,
    onRemoveAccount: () -> Unit,
) {
    var showRemoveWarn by remember { mutableStateOf(false) }

    Column {
        ListItem(
            headlineText = { Text("Email") },
            trailingContent = { Text(mavinote.email, style = Typography.bodyMedium) },
            modifier = Modifier.padding(0.dp, 0.dp, 0.dp, 20.dp),
        )

        ListItem(
            headlineText = { Text("Devices") },
            trailingContent = {
                Icon(Icons.Filled.KeyboardArrowRight, contentDescription = null)
            },
            modifier = Modifier.clickable { navController.navigate("devices?accountId=$accountId") }
        )

        Divider()

        ListItem(
            headlineText = {
                Text(
                    "Remove Account From Device",
                    color = MaterialTheme.colorScheme.error
                )
            },
            modifier = Modifier.clickable { showRemoveWarn = true }
        )

        Divider()

        ListItem(
            headlineText = { Text("Close Account", color = MaterialTheme.colorScheme.error) },
            trailingContent = {
                Icon(
                    Icons.Filled.KeyboardArrowRight,
                    contentDescription = null,
                    tint = MaterialTheme.colorScheme.error
                )
            },
            modifier = Modifier.clickable { navController.navigate("account-close?accountId=$accountId") }
        )
    }

    if (showRemoveWarn) {
        AlertDialog(
            onDismissRequest = { showRemoveWarn = false },
            text = { Text("Removing account will only remove it from this device. Are you sure about removing the account from this device?") },
            confirmButton = {
                Button(
                    modifier = Modifier.fillMaxWidth(),
                    colors = ButtonDefaults.buttonColors(containerColor = MaterialTheme.colorScheme.error),
                    onClick = {
                        showRemoveWarn = false
                        onRemoveAccount()
                    },
                ) {
                    Text("Remove Account")
                }
            }
        )
    }
}

@Preview(showBackground = true)
@Composable
fun AccountPreview() {
    val navController = rememberNavController()
    val account = Account(1, "Account on My Phone", AccountKind.Mavinote)
    val mavinote = Mavinote("email@email.com", "")

    AccountView(navController, account, mavinote) {}
}
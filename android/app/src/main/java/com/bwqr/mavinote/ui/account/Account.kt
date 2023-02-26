package com.bwqr.mavinote.ui.account

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.KeyboardArrowRight
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.font.FontWeight
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
import com.bwqr.mavinote.viewmodels.NoteViewModel
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

    fun closeAccount() {
        if (inProgress) {
            return
        }

        inProgress = true

        scope.launch {
            try {
                AccountViewModel.closeAccount(accountId)
                Bus.message("Account is closed")
                navController.navigateUp()
            } catch (e: NoteError) {
                e.handle()
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
                        mavinote = NoteViewModel.mavinoteAccount(it.id)
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
            { removeAccount() },
            { closeAccount() }
        )
    }
}

@Composable
fun AccountView(
    navController: NavController,
    account: Account,
    mavinote: Mavinote?,
    onRemoveAccount: () -> Unit,
    onCloseAccount: () -> Unit,
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
            Row(
                modifier = Modifier
                    .fillMaxWidth()
                    .padding(8.dp, 20.dp)
            ) {
                Text(
                    text = stringResource(R.string.name),
                    style = Typography.labelMedium,
                    fontWeight = FontWeight.Bold,
                    modifier = Modifier.weight(1f)
                )
                Text(
                    account.name,
                    modifier = Modifier.weight(1f)
                )
            }

            Divider()

            Row(
                modifier = Modifier
                    .fillMaxWidth()
                    .padding(8.dp, 20.dp)
            ) {
                Text(
                    text = stringResource(R.string.kind),
                    style = Typography.labelMedium,
                    fontWeight = FontWeight.Bold,
                    modifier = Modifier.weight(1f)
                )
                Text(
                    account.kind.toString(),
                    modifier = Modifier.weight(1f)
                )
            }
        }

        mavinote?.let {
            Divider()

            MavinoteAccountView(navController, account.id, it, onRemoveAccount, onCloseAccount)
        }
    }
}

@Composable
fun MavinoteAccountView(
    navController: NavController,
    accountId: Int,
    mavinote: Mavinote,
    onRemoveAccount: () -> Unit,
    onCloseAccount: () -> Unit
) {
    var showRemoveWarn by remember { mutableStateOf(false) }
    var showCloseWarn by remember { mutableStateOf(false) }

    Column {
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .padding(8.dp, 20.dp)
        ) {
            Text(
                text = "Email",
                style = Typography.labelMedium,
                fontWeight = FontWeight.Bold,
                modifier = Modifier.weight(1f)
            )

            Text(
                mavinote.email,
                modifier = Modifier.weight(1f)
            )
        }

        Row(
            modifier = Modifier
                .fillMaxWidth()
                .padding(0.dp, 16.dp, 0.dp, 0.dp)
                .clickable { navController.navigate("devices?accountId=$accountId") }
                .padding(8.dp, 20.dp),
            verticalAlignment = Alignment.CenterVertically
        ) {
            Text(
                "Devices",
                modifier = Modifier
                    .weight(1f),
            )

            Icon(Icons.Filled.KeyboardArrowRight, contentDescription = null)
        }

        Divider()

        Row(
            modifier = Modifier
                .fillMaxWidth()
                .clickable { showRemoveWarn = true }
                .padding(8.dp, 20.dp),
            verticalAlignment = Alignment.CenterVertically
        ) {
            Text(
                "Remove Account From Device",
                modifier = Modifier
                    .weight(1f),
                color = MaterialTheme.colorScheme.error
            )
        }

        Divider()

        Row(
            modifier = Modifier
                .fillMaxWidth()
                .clickable { showCloseWarn = true }
                .padding(8.dp, 20.dp),
            verticalAlignment = Alignment.CenterVertically
        ) {
            Text(
                "Close Account",
                modifier = Modifier
                    .weight(1f),
                color = MaterialTheme.colorScheme.error
            )
        }
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

    if (showCloseWarn) {
        AlertDialog(
            onDismissRequest = { showCloseWarn = false },
            text = { Text("Closing account will permanently delete it and other related information from the server and other devices. Are you sure about closing this account?") },
            confirmButton = {
                Button(
                    modifier = Modifier.fillMaxWidth(),
                    colors = ButtonDefaults.buttonColors(containerColor = MaterialTheme.colorScheme.error),
                    onClick = {
                        showCloseWarn = false
                        onCloseAccount()
                    },
                ) {
                    Text("Close Account")
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

    AccountView(navController, account, mavinote, {}) {}
}
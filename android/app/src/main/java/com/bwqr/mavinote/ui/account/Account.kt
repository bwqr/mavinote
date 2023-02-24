package com.bwqr.mavinote.ui.account

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Add
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.navigation.NavController
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

    LaunchedEffect(key1 = 0) {
        try {
            account = NoteViewModel.account(accountId)

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
        AccountView(it, mavinote) {
            if (inProgress) {
                return@AccountView
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
    }


}

@Composable
fun AccountView(
    account: Account,
    mavinote: Mavinote?,
    onRemove: () -> Unit
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

            MavinoteAccountView(it, onRemove)
        }

    }
}

@Composable
fun MavinoteAccountView(mavinote: Mavinote, onRemove: () -> Unit) {
    var showRemoveWarn by remember { mutableStateOf(false) }

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

        Divider(modifier = Modifier.padding(0.dp, 16.dp), thickness = 2.dp)

        Row(
            modifier = Modifier
                .fillMaxWidth()
                .clickable { showRemoveWarn = true }
                .padding(8.dp, 20.dp),
            verticalAlignment = Alignment.CenterVertically
        ) {
            Text(
                "Remove Account",
                modifier = Modifier
                    .weight(1f),
                color = MaterialTheme.colors.error
            )
        }
    }

    if (showRemoveWarn) {
        AlertDialog(
            onDismissRequest = { showRemoveWarn = false },
            text = { Text("Are you sure about removing account with ${mavinote.email} email address?") },
            buttons = {
                Box(contentAlignment = Alignment.Center, modifier = Modifier.fillMaxWidth()) {
                    Button(
                        onClick = {
                            showRemoveWarn = false
                            onRemove()
                        },
                        colors = ButtonDefaults.buttonColors(backgroundColor = MaterialTheme.colors.error)
                    ) {
                        Text("Remove")
                    }
                }
            }
        )
    }
}

@Composable
fun AccountFab(navController: NavController, accountId: Int) {
    var mavinoteAccount by remember { mutableStateOf(false) }

    LaunchedEffect(key1 = 0) {
        try {
            NoteViewModel.account(accountId)?.let { mavinoteAccount = it.kind == AccountKind.Mavinote }
        } catch (e: NoteError) {
            e.handle()
        }
    }

    if (mavinoteAccount) {
        ExtendedFloatingActionButton(
            text = { Text("Device") },
            icon = { Icon(Icons.Filled.Add, contentDescription = null) },
            onClick = { navController.navigate("device-add?accountId=$accountId") }
        )
    }
}

@Preview(showBackground = true)
@Composable
fun AccountPreview() {
    val account = Account(1, "Account on My Phone", AccountKind.Mavinote)
    val mavinote = Mavinote("email@email.com", "")

    AccountView(account, mavinote) {}
}
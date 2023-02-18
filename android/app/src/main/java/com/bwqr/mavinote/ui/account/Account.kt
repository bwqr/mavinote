package com.bwqr.mavinote.ui.account

import android.util.Log
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Add
import androidx.compose.material.icons.filled.MoreVert
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.navigation.NavController
import androidx.navigation.compose.rememberNavController
import com.bwqr.mavinote.R
import com.bwqr.mavinote.models.Account
import com.bwqr.mavinote.models.AccountKind
import com.bwqr.mavinote.models.Mavinote
import com.bwqr.mavinote.models.NoteError
import com.bwqr.mavinote.ui.Screen
import com.bwqr.mavinote.ui.Title
import com.bwqr.mavinote.ui.theme.Typography
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

            if (account == null) {
                Log.e("Account", "accountId $accountId does not exist")
            }
        } catch (e: NoteError) {
            e.handle()
        }
    }

    account?.let { it ->
        AccountView(navController, it, mavinote) {
            if (inProgress) {
                return@AccountView
            }

            inProgress = true

            scope.launch {
                try {
                    NoteViewModel.deleteAccount(accountId)

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
fun AccountView(
    navController: NavController,
    account: Account,
    mavinote: Mavinote?,
    onDelete: () -> Unit
) {
    var expanded by remember { mutableStateOf(false) }

    Column(modifier = Modifier.padding(12.dp)) {
        Row(verticalAlignment = Alignment.CenterVertically, modifier = Modifier.fillMaxWidth()) {
            Title(account.name, modifier = Modifier.weight(1f))
            IconButton(onClick = { expanded = true }) {
                Icon(imageVector = Icons.Filled.MoreVert, contentDescription = null)
                DropdownMenu(expanded, onDismissRequest = { expanded = false }) {
                    DropdownMenuItem(onClick = onDelete) {
                        Text(text = stringResource(R.string.remove))
                    }
                }
            }
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
                    style = Typography.subtitle1,
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
                    style = Typography.subtitle1,
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

            MavinoteAccountView(navController, account.id, it)
        }
    }
}

@Composable
fun MavinoteAccountView(navController: NavController, accountId: Int, mavinote: Mavinote) {
    Column {
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .padding(8.dp, 20.dp)
        ) {
            Text(
                text = "Email",
                style = Typography.subtitle1,
                fontWeight = FontWeight.Bold,
                modifier = Modifier.weight(1f)
            )

            Text(
                mavinote.email,
                modifier = Modifier.weight(1f)
            )
        }
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
    val navController = rememberNavController()
    val account = Account(1, "Account on My Phone", AccountKind.Mavinote)
    val mavinote = Mavinote("email@email.com", "")

    AccountView(navController, account, mavinote) {}
}
package com.bwqr.mavinote.ui.note

import android.util.Log
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.MoreVert
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.navigation.NavController
import com.bwqr.mavinote.R
import com.bwqr.mavinote.models.Account
import com.bwqr.mavinote.models.AccountKind
import com.bwqr.mavinote.models.Error
import com.bwqr.mavinote.models.Mavinote
import com.bwqr.mavinote.ui.Title
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
                    } catch (e: Error) {
                        e.handle()
                    }
                }
            }

            if (account == null) {
                Log.e("Account", "accountId $accountId does not exist")
            }
        } catch (e: Error) {
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
                    NoteViewModel.deleteAccount(accountId)

                    navController.navigateUp()
                } catch (e: Error) {
                    e.handle()
                } finally {
                    inProgress = false
                }
            }
        }
    }

}

@Composable
fun AccountView(account: Account, mavinote: Mavinote?, onDelete: () -> Unit) {
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

        Card(
            elevation = 1.dp,
            modifier = Modifier
                .padding(24.dp, 0.dp, 0.dp, 18.dp)
                .fillMaxWidth()
        ) {
            Column {
                Row(modifier = Modifier.padding(18.dp)) {
                    Text(
                        text = stringResource(R.string.name),
                        fontWeight = FontWeight.Bold,
                        modifier = Modifier.weight(1f)
                    )
                    Text(
                        account.name,
                        modifier = Modifier.weight(1f)
                    )
                }

                Row(modifier = Modifier.padding(18.dp)) {
                    Text(
                        text = stringResource(R.string.kind),
                        fontWeight = FontWeight.Bold,
                        modifier = Modifier.weight(1f)
                    )
                    Text(
                        account.kind.toString(),
                        modifier = Modifier.weight(1f)
                    )
                }
            }
        }

        mavinote?.let {
            MavinoteAccountView(it)
        }
    }
}

@Composable
fun MavinoteAccountView(mavinote: Mavinote) {
    Card(
        elevation = 1.dp,
        modifier = Modifier
            .padding(24.dp, 0.dp, 0.dp, 18.dp)
            .fillMaxWidth()
    ) {
        Column {
            Row(modifier = Modifier.padding(18.dp)) {
                Text(
                    text = "Email",
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
}

@Preview(showBackground = true)
@Composable
fun AccountPreview() {
    val account = Account(1, "Note on My Phone", AccountKind.Local)
    val mavinote = Mavinote("email@email.com", "")

    AccountView(account, mavinote) {}
}
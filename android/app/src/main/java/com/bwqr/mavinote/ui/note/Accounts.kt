package com.bwqr.mavinote.ui.note

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material.Card
import androidx.compose.material.FloatingActionButton
import androidx.compose.material.Icon
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Add
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.navigation.NavController
import androidx.navigation.compose.rememberNavController
import com.bwqr.mavinote.R
import com.bwqr.mavinote.models.Account
import com.bwqr.mavinote.models.AccountKind
import com.bwqr.mavinote.models.Error
import com.bwqr.mavinote.ui.NoteScreens
import com.bwqr.mavinote.ui.Title
import com.bwqr.mavinote.viewmodels.NoteViewModel
import kotlinx.coroutines.flow.catch
import kotlinx.coroutines.flow.launchIn
import kotlinx.coroutines.flow.onEach

@Composable
fun Accounts(navController: NavController) {
    var accounts by remember { mutableStateOf(listOf<Account>()) }

    LaunchedEffect(key1 = 0) {
        NoteViewModel
            .accounts()
            .onEach { accounts = it }
            .catch {
                when (val cause = it.cause) {
                    is Error -> cause.handle()
                }
            }
            .launchIn(this)
    }

    AccountsView(navController, accounts)
}

@Composable
fun AccountsView(navController: NavController, accounts: List<Account>) {
    Column(modifier = Modifier.padding(12.dp)) {
        Title(
            stringResource(R.string.accounts),
            modifier = Modifier.padding(0.dp, 0.dp, 0.dp, 32.dp)
        )

        Card(
            elevation = 1.dp,
            modifier = Modifier
                .padding(24.dp, 0.dp, 0.dp, 0.dp)
                .fillMaxWidth()
                .padding(0.dp, 0.dp, 0.dp, 18.dp)
        ) {
            LazyColumn {
                items(accounts) { account ->
                    Text(
                        account.name,
                        modifier = Modifier
                            .fillMaxWidth()
                            .clickable { navController.navigate("account/${account.id}") }
                            .padding(16.dp, 12.dp)
                    )
                }
            }
        }
    }
}

@Composable
fun AccountsFab(navController: NavController) {
    FloatingActionButton(onClick = { navController.navigate(NoteScreens.AccountAdd.route) }) {
        Icon(Icons.Filled.Add, contentDescription = null)
    }
}

@Preview(showBackground = true)
@Composable
fun AccountsPreview() {
    val navController = rememberNavController()

    val accounts = listOf(
        Account(1, "Default", AccountKind.Local),
        Account(2, "Remote", AccountKind.Mavinote),
    )

    AccountsView(navController, accounts)
}
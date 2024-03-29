package com.bwqr.mavinote.ui.account

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Add
import androidx.compose.material.icons.filled.KeyboardArrowRight
import androidx.compose.material3.Divider
import androidx.compose.material3.ExtendedFloatingActionButton
import androidx.compose.material3.Icon
import androidx.compose.material3.ListItem
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.navigation.NavController
import androidx.navigation.compose.rememberNavController
import com.bwqr.mavinote.R
import com.bwqr.mavinote.models.Account
import com.bwqr.mavinote.models.AccountKind
import com.bwqr.mavinote.models.NoteError
import com.bwqr.mavinote.ui.Screen
import com.bwqr.mavinote.ui.theme.MavinoteTheme
import com.bwqr.mavinote.ui.util.Title
import com.bwqr.mavinote.viewmodels.AccountViewModel
import kotlinx.coroutines.flow.catch
import kotlinx.coroutines.flow.launchIn
import kotlinx.coroutines.flow.onEach

@Composable
fun Accounts(navController: NavController) {
    var accounts by remember { mutableStateOf(listOf<Account>()) }

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

    AccountsView(navController, accounts)
}

@Composable
fun AccountsView(navController: NavController, accounts: List<Account>) {
    Column(
        verticalArrangement = Arrangement.spacedBy(24.dp),
        modifier = Modifier.padding(16.dp)
    ) {
        Title(stringResource(R.string.accounts))


        LazyColumn {
            items(accounts.mapIndexed { index, account ->
                Pair(
                    index,
                    account
                )
            }) { (index, account) ->
                ListItem(
                    headlineContent = { Text(account.name) },
                    trailingContent = {
                        Icon(Icons.Filled.KeyboardArrowRight, contentDescription = null)
                    },
                    modifier = Modifier.clickable { navController.navigate("account/${account.id}") }
                )

                if (index != accounts.size - 1) {
                    Divider()
                }
            }
        }
    }
}

@Composable
fun AccountsFab(navController: NavController) {
    ExtendedFloatingActionButton(
        text = { Text("Account") },
        icon = { Icon(Icons.Filled.Add, contentDescription = null) },
        onClick = { navController.navigate(Screen.Account.AccountAdd.route) }
    )
}

@Preview(showBackground = true)
@Composable
fun AccountsPreview() {
    val navController = rememberNavController()

    val accounts = listOf(
        Account(1, "Default", AccountKind.Local),
        Account(2, "Remote", AccountKind.Mavinote),
        Account(2, "Home", AccountKind.Mavinote),
        Account(2, "Work", AccountKind.Mavinote),
    )

    MavinoteTheme {
        AccountsView(navController, accounts)
    }
}
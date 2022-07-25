package com.bwqr.mavinote.ui.note

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material.Button
import androidx.compose.material.Text
import androidx.compose.runtime.*
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.ExperimentalUnitApi
import androidx.compose.ui.unit.TextUnit
import androidx.compose.ui.unit.TextUnitType
import androidx.navigation.NavController
import com.bwqr.mavinote.models.Account
import com.bwqr.mavinote.models.AccountKind
import com.bwqr.mavinote.models.ReaxException
import com.bwqr.mavinote.ui.NoteScreens
import com.bwqr.mavinote.viewmodels.NoteViewModel
import kotlinx.coroutines.flow.catch
import kotlinx.coroutines.flow.launchIn
import kotlinx.coroutines.flow.onEach
import kotlinx.coroutines.launch

@OptIn(ExperimentalUnitApi::class)
@Composable
fun Accounts(navController: NavController) {
    val coroutineScope = rememberCoroutineScope()

    var accounts by remember { mutableStateOf(listOf<Account>()) }
    var inProgress by remember { mutableStateOf(false) }

    LaunchedEffect(key1 = 0) {
        NoteViewModel()
            .accounts()
            .onEach { accounts = it }
            .catch {
                when (val cause = it.cause) {
                    is ReaxException -> cause.handle()
                }
            }
            .launchIn(this)
    }

    Column {
        Text(
            text = "Accounts",
            fontWeight = FontWeight.ExtraBold,
            fontSize = TextUnit(6f, TextUnitType.Em)
        )

        LazyColumn {
            items(accounts) { account ->
                Row {
                    Text(account.kind.toString())

                    if (account.kind == AccountKind.Mavinote) {
                        Button(onClick = {
                            if (inProgress) {
                                return@Button
                            }

                            inProgress = true

                            coroutineScope.launch {
                                try {
                                    NoteViewModel().deleteAccount(account.id)
                                } catch(e: ReaxException) {
                                    e.handle()
                                } finally {
                                    inProgress = false
                                }
                            }
                        }) {
                            Text("Delete")
                        }
                    }
                }
            }
        }

        Button(onClick = { navController.navigate(NoteScreens.AccountAdd.route) }) {
            Text("Add Mavinote account")
        }
    }
}
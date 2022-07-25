package com.bwqr.mavinote.ui.note

import androidx.compose.foundation.layout.Column
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
import com.bwqr.mavinote.models.ReaxException
import com.bwqr.mavinote.ui.NoteScreens
import com.bwqr.mavinote.viewmodels.NoteViewModel
import kotlinx.coroutines.flow.catch
import kotlinx.coroutines.flow.launchIn
import kotlinx.coroutines.flow.onEach

@OptIn(ExperimentalUnitApi::class)
@Composable
fun Accounts(navController: NavController) {
    var accounts by remember { mutableStateOf(listOf<Account>()) }

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
                Text(account.kind.toString())
            }
        }

        Button(onClick = { navController.navigate(NoteScreens.AccountAdd.route) }) {
            Text("Add Mavinote account")
        }
    }
}
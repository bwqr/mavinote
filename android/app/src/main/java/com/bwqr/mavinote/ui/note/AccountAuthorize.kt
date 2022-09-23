package com.bwqr.mavinote.ui.note

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.text.input.PasswordVisualTransformation
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import com.bwqr.mavinote.models.*
import com.bwqr.mavinote.viewmodels.NoteViewModel
import kotlinx.coroutines.launch

@Composable
fun AccountAuthorize(accountId: Int, onClose: () -> Unit) {
    val scope = rememberCoroutineScope()

    var account by remember { mutableStateOf<Account?>(null) }
    var mavinote by remember { mutableStateOf<Mavinote?>(null) }
    var inProgress by remember { mutableStateOf(false) }
    var error by remember { mutableStateOf<String?>(null) }

    LaunchedEffect(key1 = accountId) {
        try {
            account = NoteViewModel.account(accountId)
            mavinote = NoteViewModel.mavinoteAccount(accountId)
        } catch (e: Error) {
            e.handle()
        }
    }

    account?.let { account ->
        mavinote?.let { mavinote ->
            AccountAuthorizeView(account, mavinote, error, onClose) { password ->
                if (inProgress) {
                    return@AccountAuthorizeView
                }

                error = null

                if (password.isBlank()) {
                    error = "Please type your password"
                    return@AccountAuthorizeView
                }

                inProgress = true

                scope.launch {
                    try {
                        NoteViewModel.authorizeMavinoteAccount(accountId, password)

                        onClose()
                    } catch (e: Error) {
                        when (e) {
                            is HttpError.Unauthorized -> error = "Wrong password, please try again"
                            is Message -> error = e.message
                            else -> e.handle()
                        }
                    } finally {
                        inProgress = false
                    }
                }
            }
        }
    }
}

@Composable
fun AccountAuthorizeView(
    account: Account,
    mavinote: Mavinote,
    error: String?,
    onClose: () -> Unit,
    onAuthorize: (password: String) -> Unit
) {
    var password by remember { mutableStateOf("") }

    AlertDialog(
        onDismissRequest = { },
        text = {
            Column {
                Text(
                    text = "${account.name} account with ${mavinote.email} email needs authorization",
                    modifier = Modifier.padding(0.dp, 0.dp, 0.dp, 16.dp)
                )

                Text(
                    text = "Please type your Password",
                    modifier = Modifier.padding(0.dp, 0.dp, 0.dp, 8.dp)
                )

                TextField(
                    value = password,
                    onValueChange = { password = it },
                    keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Password),
                    visualTransformation = PasswordVisualTransformation()
                )

                if (error != null) {
                    Text(
                        text = error,
                        color = MaterialTheme.colors.error,
                    )
                }
            }
        },
        buttons = {
            Row(
                horizontalArrangement = Arrangement.SpaceEvenly,
                modifier = Modifier.fillMaxWidth()
            ) {
                TextButton(onClick = onClose) {
                    Text(text = "Cancel")
                }

                TextButton(onClick = { onAuthorize(password) }) {
                    Text(text = "Authorize")
                }
            }
        }
    )
}

@Preview(showBackground = true)
@Composable
fun AccountAuthorizePreview() {
    val account = Account(1, "Remote Account", AccountKind.Mavinote)
    val mavinote = Mavinote("email@address.com", "token")
    val error = "Please fill password field"

    AccountAuthorizeView(account, mavinote, error, {}) { _ -> }
}
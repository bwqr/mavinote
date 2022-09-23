package com.bwqr.mavinote.ui.note

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.text.input.PasswordVisualTransformation
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.navigation.NavController
import com.bwqr.mavinote.R
import com.bwqr.mavinote.models.Error
import com.bwqr.mavinote.models.HttpError
import com.bwqr.mavinote.models.Message
import com.bwqr.mavinote.ui.Title
import com.bwqr.mavinote.viewmodels.NoteViewModel
import kotlinx.coroutines.launch

@Composable
fun AccountAdd(navController: NavController) {
    val scope = rememberCoroutineScope()
    var inProgress by remember { mutableStateOf(false) }
    var error by remember { mutableStateOf<String?>(null) }

    AccountAddView(error) { accountName, email, password, createAccount ->
        if (inProgress) {
            return@AccountAddView
        }

        error = null

        if (accountName.isBlank() || email.isBlank() || password.isBlank()) {
            error = "Please fill the fields"
            return@AccountAddView
        }

        inProgress = true

        scope.launch {
            try {
                NoteViewModel.addAccount(accountName, email, password, createAccount)

                navController.navigateUp()
            } catch (e: Error) {
                when (e) {
                    is Message -> error = e.message
                    is HttpError.Unauthorized -> error =
                            "Invalid credentials. If you do not have a Mavinote account, you can create a new one by checking the box above"
                    else -> e.handle()
                }

            } finally {
                inProgress = false
            }
        }
    }
}

@Composable
fun AccountAddView(
    error: String?,
    onAddAccount: (accountName: String, email: String, password: String, createAccount: Boolean) -> Unit
) {
    var accountName by remember { mutableStateOf("") }
    var email by remember { mutableStateOf("") }
    var password by remember { mutableStateOf("") }
    var createAccount by remember { mutableStateOf(false) }

    val scrollState = rememberScrollState()

    Column(modifier = Modifier.padding(12.dp)) {
        Title(
            stringResource(R.string.add_account),
            modifier = Modifier.padding(0.dp, 0.dp, 0.dp, 32.dp)
        )

        Column(
            modifier = Modifier
                .padding(24.dp, 0.dp)
                .verticalScroll(scrollState)
                .weight(1f, true)
        ) {

            Text(
                text = stringResource(R.string.account_add_description),
                modifier = Modifier.padding(0.dp, 0.dp, 0.dp, 16.dp)
            )

            Column(modifier = Modifier.padding(0.dp, 0.dp, 0.dp, 32.dp)) {
                Text(
                    text = stringResource(R.string.account_name),
                    modifier = Modifier.padding(0.dp, 0.dp, 0.dp, 12.dp)
                )
                TextField(
                    value = accountName,
                    onValueChange = { accountName = it },
                    modifier = Modifier.fillMaxWidth()
                )
            }

            Column(modifier = Modifier.padding(0.dp, 0.dp, 0.dp, 32.dp)) {
                Text(
                    text = stringResource(R.string.email),
                    modifier = Modifier.padding(0.dp, 0.dp, 0.dp, 12.dp)
                )
                TextField(
                    value = email,
                    onValueChange = { email = it },
                    modifier = Modifier.fillMaxWidth(),
                    keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Email),
                )
            }

            Column(modifier = Modifier.padding(0.dp, 0.dp, 0.dp, 32.dp)) {
                Text(
                    text = stringResource(R.string.password),
                    modifier = Modifier.padding(0.dp, 0.dp, 0.dp, 12.dp),

                )
                TextField(
                    value = password,
                    onValueChange = { password = it },
                    modifier = Modifier.fillMaxWidth(),
                    keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Password),
                    visualTransformation = PasswordVisualTransformation(),
                )
            }

            Row(
                verticalAlignment = Alignment.CenterVertically,
                modifier = Modifier.padding(0.dp, 0.dp, 0.dp, 32.dp)
            ) {
                Checkbox(checked = createAccount, onCheckedChange = { createAccount = it })
                Text(text = stringResource(R.string.dont_have_account_create_one))
            }

            if (error != null) {
                Text(
                    text = error,
                    color = MaterialTheme.colors.error,
                    modifier = Modifier.padding(0.dp, 0.dp, 0.dp, 16.dp)
                )
            }
        }

        Button(
            onClick = { onAddAccount(accountName, email, password, createAccount) },
            modifier = Modifier.fillMaxWidth()
        ) {
            Text(text = stringResource(id = R.string.add_account))
        }
    }
}

@Preview(showBackground = true)
@Composable
fun AccountAddPreview() {
    val error = "Please fill the fields"

    AccountAddView(error) { _, _, _, _ -> }
}
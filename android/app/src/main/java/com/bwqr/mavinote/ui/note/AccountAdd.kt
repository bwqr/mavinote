package com.bwqr.mavinote.ui.note

import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material.Button
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.material.TextField
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.navigation.NavController
import com.bwqr.mavinote.R
import com.bwqr.mavinote.models.Message
import com.bwqr.mavinote.models.ReaxException
import com.bwqr.mavinote.ui.Title
import com.bwqr.mavinote.viewmodels.NoteViewModel
import kotlinx.coroutines.launch

@Composable
fun AccountAdd(navController: NavController) {
    val scope = rememberCoroutineScope()
    var inProgress by remember { mutableStateOf(false) }
    var error by remember { mutableStateOf<String?>(null) }

    CreateAccountView(error) { accountName, email, password ->
        if (inProgress) {
            return@CreateAccountView
        }

        error = null

        if (accountName.isBlank() || email.isBlank() || password.isBlank()) {
            error = "Please fill the fields"
            return@CreateAccountView
        }

        inProgress = true

        scope.launch {
            try {
                NoteViewModel().createAccount(accountName, email, password)

                navController.navigateUp()
            } catch (e: ReaxException) {
                if (e.error is Message) {
                    error = e.error.message
                } else {
                    e.handle()
                }

            } finally {
                inProgress = false
            }
        }
    }
}

@Composable
fun CreateAccountView(
    error: String?,
    onCreateAccount: (accountName: String, email: String, password: String) -> Unit
) {
    var accountName by remember { mutableStateOf("") }
    var email by remember { mutableStateOf("") }
    var password by remember { mutableStateOf("") }

    Column(modifier = Modifier.padding(12.dp)) {
        Title(
            stringResource(R.string.create_account),
            modifier = Modifier.padding(0.dp, 0.dp, 0.dp, 48.dp)
        )

        Column(
            modifier = Modifier
                .padding(40.dp, 0.dp, 0.dp, 0.dp)
        ) {
            Column(modifier = Modifier.padding(0.dp, 0.dp, 0.dp, 32.dp)) {
                Text(
                    text = stringResource(R.string.account_name),
                    modifier = Modifier.padding(0.dp, 0.dp, 0.dp, 12.dp)
                )
                TextField(value = accountName, onValueChange = { accountName = it })
            }

            Column(modifier = Modifier.padding(0.dp, 0.dp, 0.dp, 32.dp)) {
                Text(
                    text = stringResource(R.string.email),
                    modifier = Modifier.padding(0.dp, 0.dp, 0.dp, 12.dp)
                )
                TextField(value = email, onValueChange = { email = it })
            }

            Column(modifier = Modifier.padding(0.dp, 0.dp, 0.dp, 32.dp)) {
                Text(
                    text = stringResource(R.string.password),
                    modifier = Modifier.padding(0.dp, 0.dp, 0.dp, 12.dp)
                )
                TextField(value = password, onValueChange = { password = it })
            }

            if (error != null) {
                Text(
                    text = error,
                    color = MaterialTheme.colors.error,
                    modifier = Modifier.padding(0.dp, 0.dp, 0.dp, 32.dp)
                )
            }

            Box(contentAlignment = Alignment.BottomCenter, modifier = Modifier.weight(1f)) {
                Button(
                    onClick = { onCreateAccount(accountName, email, password) },
                    modifier = Modifier.fillMaxWidth()
                ) {
                    Text(text = stringResource(id = R.string.create_account))
                }
            }
        }
    }
}

@Preview(showBackground = true)
@Composable
fun CreateAccountPreview() {
    val error = null

    CreateAccountView(error) { _, _, _ -> }
}
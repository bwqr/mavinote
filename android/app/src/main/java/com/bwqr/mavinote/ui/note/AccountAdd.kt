package com.bwqr.mavinote.ui.note

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.navigation.NavController
import com.bwqr.mavinote.R
import com.bwqr.mavinote.models.MavinoteError
import com.bwqr.mavinote.models.NoteError
import com.bwqr.mavinote.viewmodels.NoteViewModel
import kotlinx.coroutines.launch

sealed class Step {
    object SendCode: Step()
    object VerifyCode: Step()
}

@Composable
fun AccountAdd(navController: NavController) {
    val scope = rememberCoroutineScope()
    var inProgress by remember { mutableStateOf(false) }
    var error by remember { mutableStateOf<String?>(null) }

    var name by remember { mutableStateOf("") }
    var email by remember { mutableStateOf("") }
    var step by remember { mutableStateOf<Step>(Step.SendCode) }

    when(step) {
        is Step.SendCode -> SendCodeView(error, name, email) { sentName, sentEmail ->
            if (inProgress) {
                return@SendCodeView
            }

            error = null

            if (sentName.isBlank()) {
                error = "Please type a name"
                return@SendCodeView
            }

            if (sentEmail.isBlank()) {
                error = "Please type a valid email"
                return@SendCodeView
            }

            inProgress = true
            name = sentName
            email = sentEmail

            scope.launch {
                try {
                    NoteViewModel.sendCode(sentEmail)
                    step = Step.VerifyCode
                } catch (e: NoteError) {
                    if (e is MavinoteError.Message && e.message == "user_exists") {
                        error = "This email is already used"
                    } else {
                        e.handle()
                    }
                } finally {
                    inProgress = false
                }
            }
        }
        is Step.VerifyCode -> VerifyCodeView(error, {
            if (inProgress) {
                return@VerifyCodeView
            }

            error = null

            if (it.isBlank()) {
                error = "Please type the code"
                return@VerifyCodeView
            }

            inProgress = true

            scope.launch {
                try {
                    NoteViewModel.signUp(name, email, it)
                } catch (e: NoteError) {
                    if (e is MavinoteError.Message && e.message == "invalid_code") {
                        error = "Code is invalid"
                    } else {
                        e.handle()
                    }
                } finally {
                    inProgress = false
                }
            }
        }) { step = Step.SendCode }
    }

}

@Composable
fun SendCodeView(error: String?, initialName: String, initialEmail: String, onSendCode: (name: String, email: String) -> Unit) {
    var name by remember { mutableStateOf(initialName) }
    var email by remember { mutableStateOf(initialEmail) }

    val scrollState = rememberScrollState()

    Column(modifier = Modifier.padding(12.dp).verticalScroll(scrollState)) {
        Text(
            text = stringResource(R.string.name),
            modifier = Modifier.padding(0.dp, 0.dp, 0.dp, 12.dp)
        )
        TextField(
            value = name,
            onValueChange = { name = it },
            modifier = Modifier.fillMaxWidth()
        )

        Text(
            text = stringResource(R.string.email),
            modifier = Modifier.padding(0.dp, 0.dp, 0.dp, 12.dp)
        )
        TextField(
            value = email,
            onValueChange = { email = it },
            modifier = Modifier.fillMaxWidth()
        )

        if (error != null) {
            Text(
                text = error,
                color = MaterialTheme.colors.error,
                modifier = Modifier.padding(0.dp, 0.dp, 0.dp, 16.dp)
            )
        }

        Button(
            onClick = { onSendCode(name, email) },
            modifier = Modifier.fillMaxWidth()
        ) {
            Text(text = "Send Verification Code")
        }
    }
}

@Composable
fun VerifyCodeView(error: String?, onVerify: (code: String) -> Unit, onBack: () -> Unit) {
    var code by remember { mutableStateOf("") }

    val scrollState = rememberScrollState()

    Column(modifier = Modifier.padding(12.dp).verticalScroll(scrollState)) {
        Column(modifier = Modifier.padding(0.dp, 0.dp, 0.dp, 32.dp)) {
            Text(
                text = "Code",
                modifier = Modifier.padding(0.dp, 0.dp, 0.dp, 12.dp)
            )
            TextField(
                value = code,
                onValueChange = { code = it },
                modifier = Modifier.fillMaxWidth()
            )

            if (error != null) {
                Text(
                    text = error,
                    color = MaterialTheme.colors.error,
                    modifier = Modifier.padding(0.dp, 0.dp, 0.dp, 16.dp)
                )
            }

            Button(
                onClick = { onBack() },
                modifier = Modifier.fillMaxWidth()
            ) {
                Text(text = "Back")
            }

            Button(
                onClick = { onVerify(code) },
                modifier = Modifier.fillMaxWidth()
            ) {
                Text(text = "Verify Code")
            }
        }
    }
}

@Preview(showBackground = true)
@Composable
fun AccountAddPreview() {
    val error = null

    Column() {
        SendCodeView(error, "My name", "My email") { _, _ -> }
        VerifyCodeView(error, { }) { }
    }
}
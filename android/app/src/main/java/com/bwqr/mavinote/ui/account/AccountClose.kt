package com.bwqr.mavinote.ui.account

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.navigation.NavController
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import com.bwqr.mavinote.Bus
import com.bwqr.mavinote.R
import com.bwqr.mavinote.models.Mavinote
import com.bwqr.mavinote.models.MavinoteError
import com.bwqr.mavinote.models.NoteError
import com.bwqr.mavinote.ui.ErrorText
import com.bwqr.mavinote.ui.Screen
import com.bwqr.mavinote.ui.Title
import com.bwqr.mavinote.viewmodels.AccountViewModel
import kotlinx.coroutines.launch

sealed class AccountCloseScreen(route: String) : Screen(route) {
    object SendVerificationCode : AccountCloseScreen("send-code")
    object VerifyCode : AccountCloseScreen("verify-code")
}

@Composable
fun AccountClose(navController: NavController, accountId: Int) {
    val coroutineScope = rememberCoroutineScope()
    val accountCloseNavController = rememberNavController()
    var mavinote by remember { mutableStateOf<Mavinote?>(null) }

    LaunchedEffect(key1 = 0) {
        try {
            mavinote = AccountViewModel.mavinoteAccount(accountId)
        } catch (e: NoteError) {
            e.handle()
        }
    }

    Column(modifier = Modifier.padding(12.dp)) {
        Title(
            text = "Close Account",
            modifier = Modifier.padding(0.dp, 0.dp, 0.dp, 12.dp)
        )

        NavHost(
            accountCloseNavController,
            startDestination = AccountCloseScreen.SendVerificationCode.route
        ) {
            composable(AccountCloseScreen.SendVerificationCode.route) {
                var inProgress by remember { mutableStateOf(false) }

                SendCodeView(mavinote?.email.orEmpty(), inProgress) {
                    if (inProgress) {
                        return@SendCodeView
                    }

                    inProgress = true

                    coroutineScope.launch {
                        try {
                            AccountViewModel.sendAccountCloseCode(accountId)
                            accountCloseNavController.navigate(AccountCloseScreen.VerifyCode.route)
                        } catch (e: NoteError) {
                            e.handle()
                        } finally {
                            inProgress = false
                        }
                    }
                }
            }

            composable(AccountCloseScreen.VerifyCode.route) {
                var validationErrors by remember {
                    mutableStateOf<Set<AccountCloseValidationErrors>>(
                        setOf()
                    )
                }
                var error by remember { mutableStateOf<String?>(null) }
                var inProgress by remember { mutableStateOf(false) }

                VerifyCodeView(
                    mavinote?.email.orEmpty(),
                    inProgress,
                    error,
                    validationErrors,
                    onDismissError = { error = null }
                ) { code ->
                    if (inProgress) {
                        return@VerifyCodeView
                    }

                    val mutableValidationErrors = mutableSetOf<AccountCloseValidationErrors>()

                    if (code.isBlank()) {
                        mutableValidationErrors.add(AccountCloseValidationErrors.InvalidCode)
                    }

                    validationErrors = mutableValidationErrors
                    if (validationErrors.isNotEmpty()) {
                        return@VerifyCodeView
                    }

                    inProgress = true

                    coroutineScope.launch {
                        try {
                            AccountViewModel.closeAccount(accountId, code)
                            navController.navigateUp()
                            navController.navigateUp()
                            Bus.message("Account is closed")
                        } catch (e: NoteError) {
                            when {
                                e is MavinoteError.Message && e.message == "expired_code" -> {
                                    error = "5 minutes waiting is timed out. Please try again."
                                }
                                e is MavinoteError.Message && e.message == "invalid_code" -> {
                                    error = "You have entered invalid code. Please check the verification code."
                                }
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
}

@Composable
fun SendCodeView(
    email: String,
    inProgress: Boolean,
    onSendCode: () -> Unit
) {
    val scrollState = rememberScrollState()

    Column {
        Column(
            modifier = Modifier
                .verticalScroll(scrollState)
                .weight(1f)
        ) {
            Text(
                "Are sure about closing account?",
                modifier = Modifier.padding(0.dp, 8.dp)
            )

            Text(
                "In order to close the account, we will send a verification code to $email email address.",
                modifier = Modifier.padding(0.dp, 16.dp)
            )
        }

        Button(
            modifier = Modifier.fillMaxWidth(),
            onClick = onSendCode,
            enabled = !inProgress
        ) {
            Text("Send Verification Code")
        }
    }
}

private enum class AccountCloseValidationErrors {
    InvalidCode,
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
private fun VerifyCodeView(
    email: String,
    inProgress: Boolean,
    error: String?,
    validationErrors: Set<AccountCloseValidationErrors>,
    onDismissError: () -> Unit,
    onCloseAccount: (code: String) -> Unit
) {
    val scrollState = rememberScrollState()
    var code by remember { mutableStateOf("") }

    Column {
        Column(
            modifier = Modifier
                .verticalScroll(scrollState)
                .weight(1f)
        ) {
            Text(
                "An 8 digit verification code is sent to $email email address.",
                modifier = Modifier.padding(0.dp, 8.dp)
            )

            Text(
                "Please enter verification code to close your account.",
                modifier = Modifier.padding(0.dp, 16.dp)
            )

            Text(
                text = stringResource(R.string.code),
                modifier = Modifier.padding(0.dp, 0.dp, 0.dp, 12.dp)
            )
            TextField(
                value = code,
                onValueChange = { code = it },
                modifier = Modifier
                    .fillMaxWidth()
                    .padding(0.dp, 0.dp, 0.dp, 12.dp)
            )

            if (validationErrors.contains(AccountCloseValidationErrors.InvalidCode)) {
                ErrorText(error = "Please specify the verification code")
            }
        }

        Button(
            modifier = Modifier.fillMaxWidth(),
            onClick = { onCloseAccount(code) },
            enabled = !inProgress,
            colors = ButtonDefaults.buttonColors(containerColor = MaterialTheme.colorScheme.error),
        ) {
            Text("Close Account")
        }
    }
    error?.let {
        AlertDialog(
            onDismissRequest = onDismissError,
            text = { Text(it) },
            confirmButton = { }
        )
    }
}

@Preview(showBackground = true)
@Composable
fun SendCodePreview() {
    SendCodeView("email@email.com", false) { }
}

@Preview(showBackground = true)
@Composable
fun VerifyCodePreview() {
    VerifyCodeView("email@email.com", false, null,  setOf(), { }) { }
}
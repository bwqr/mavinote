package com.bwqr.mavinote.ui.account

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.KeyboardArrowRight
import androidx.compose.material3.AlertDialog
import androidx.compose.material3.Button
import androidx.compose.material3.Divider
import androidx.compose.material3.Icon
import androidx.compose.material3.ListItem
import androidx.compose.material3.Text
import androidx.compose.material3.TextField
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.navigation.NavController
import androidx.navigation.NavType
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import androidx.navigation.navArgument
import com.bwqr.mavinote.Bus
import com.bwqr.mavinote.R
import com.bwqr.mavinote.models.MavinoteError
import com.bwqr.mavinote.models.NoteError
import com.bwqr.mavinote.models.StorageError
import com.bwqr.mavinote.ui.ErrorText
import com.bwqr.mavinote.ui.Screen
import com.bwqr.mavinote.ui.Title
import com.bwqr.mavinote.ui.theme.MavinoteTheme
import com.bwqr.mavinote.viewmodels.AccountViewModel
import kotlinx.coroutines.launch

sealed class AccountAddScreen(route: String) : Screen(route) {
    object ChooseAccountAddKind : AccountAddScreen("choose-account-add-kind")

    class AddExistingAccount(route: String) : AccountAddScreen(route) {
        object EnterAccountInfo : AccountAddScreen("enter-account-info")
        object ShowPublicKey : AccountAddScreen("show-public-key?email={email}&token={token}")
    }

    sealed class CreateAccount(route: String) : AccountAddScreen(route) {
        object SendVerificationCode : CreateAccount("send-code")
        object VerifyCode : CreateAccount("verify-code?email={email}")
    }
}

@Composable
fun AccountAdd(navController: NavController) {
    fun onAccountAdd() {
        Bus.message("Account is successfully added")
        navController.navigateUp()
    }

    val accountAddNavController = rememberNavController()

    Column(modifier = Modifier.padding(16.dp)) {
        Title(
            text = "Add Account",
            modifier = Modifier.padding(0.dp, 0.dp, 0.dp, 12.dp)
        )

        NavHost(
            accountAddNavController,
            startDestination = AccountAddScreen.ChooseAccountAddKind.route,
        ) {
            composable(AccountAddScreen.ChooseAccountAddKind.route) {
                ChooseAccountAddKind(accountAddNavController)
            }

            composable(
                AccountAddScreen.AddExistingAccount.EnterAccountInfo.route,
            ) {
                EnterAccountInfo(accountAddNavController) { onAccountAdd() }
            }

            composable(
                AccountAddScreen.AddExistingAccount.ShowPublicKey.route,
                arguments = listOf(
                    navArgument("email") { type = NavType.StringType },
                    navArgument("token") { type = NavType.StringType }
                )
            ) {
                ShowPublicKey(
                    it.arguments?.getString("email")!!,
                    it.arguments?.getString("token")!!,
                ) { onAccountAdd() }
            }

            composable(AccountAddScreen.CreateAccount.SendVerificationCode.route) {
                SendVerificationCode(accountAddNavController)
            }

            composable(
                AccountAddScreen.CreateAccount.VerifyCode.route,
                arguments = listOf(navArgument("email") { type = NavType.StringType })
            ) {
                VerifyCode(it.arguments?.getString("email")!!) {
                    Bus.message("Account is successfully created")
                    navController.navigateUp()
                }
            }
        }
    }
}

@Composable
fun ChooseAccountAddKind(navController: NavController) {
    Column {
        Text(
            "You can create a new account or add an already existing account",
            modifier = Modifier.padding(0.dp, 8.dp)
        )

        Text(
            "If you have already created an account from another device, you can also access it from this device",
            modifier = Modifier.padding(0.dp, 16.dp)
        )

        ListItem(
            headlineContent = { Text("Add an Existing Account") },
            trailingContent = { Icon(Icons.Filled.KeyboardArrowRight, contentDescription = null) },
            modifier = Modifier.clickable { navController.navigate(AccountAddScreen.AddExistingAccount.EnterAccountInfo.route) }
        )

        Divider()
        ListItem(
            headlineContent = { Text("Create a New Account") },
            trailingContent = { Icon(Icons.Filled.KeyboardArrowRight, contentDescription = null) },
            modifier = Modifier.clickable { navController.navigate(AccountAddScreen.CreateAccount.SendVerificationCode.route) }
        )
    }
}

private enum class ValidationErrors {
    InvalidEmail,
    InvalidCode,
}

@Composable
fun EnterAccountInfo(navController: NavController, onAccountAdd: () -> Unit) {
    var validationErrors by remember { mutableStateOf<Set<ValidationErrors>>(setOf()) }
    var error by remember { mutableStateOf<String?>(null) }
    var inProgress by remember { mutableStateOf(false) }

    val coroutineScope = rememberCoroutineScope()
    val scrollState = rememberScrollState()

    var email by remember { mutableStateOf("") }

    Column {
        Column(
            modifier = Modifier
                .verticalScroll(scrollState)
                .weight(1f)
        ) {
            Text(
                "Email address is used to identify accounts.",
                modifier = Modifier.padding(0.dp, 8.dp)
            )

            Text(
                "Please enter the email address of the account you want to add.",
                modifier = Modifier.padding(0.dp, 16.dp)
            )

            Text(
                text = stringResource(R.string.email),
                modifier = Modifier.padding(0.dp, 0.dp, 0.dp, 12.dp)
            )
            TextField(
                value = email,
                onValueChange = { email = it },
                modifier = Modifier
                    .fillMaxWidth()
                    .padding(0.dp, 0.dp, 0.dp, 12.dp)
            )

            if (validationErrors.contains(ValidationErrors.InvalidEmail)) {
                ErrorText(error = "Please specify a valid email")
            }
        }

        Button(
            modifier = Modifier.fillMaxWidth(),
            enabled = !inProgress,
            onClick = {
                if (inProgress) {
                    return@Button
                }

                val mutableValidationErrors = mutableSetOf<ValidationErrors>()

                if (email.isBlank()) {
                    mutableValidationErrors.add(ValidationErrors.InvalidEmail)
                }


                if (mutableValidationErrors.size != 0) {
                    validationErrors = mutableValidationErrors
                    return@Button
                }

                validationErrors = setOf()
                inProgress = true

                coroutineScope.launch {
                    try {
                        val token = AccountViewModel.requestVerification(email)
                        navController.navigate("show-public-key?email=$email&token=$token")
                    } catch (e: NoteError) {
                        when {
                            e is MavinoteError.Message && e.message == "email_not_found" -> {
                                error = "Email could not be found. Please check your input."
                            }
                            e is MavinoteError.Message && e.message == "device_already_exists" -> {
                                try {
                                    AccountViewModel.addAccount(email)
                                    onAccountAdd()
                                } catch (e: NoteError) {
                                    e.handle()
                                }
                            }
                            e is MavinoteError.Message && e.message == "device_exists_but_passwords_mismatch" -> {
                                error =
                                    "An unexpected state is occurred. A device with our public key is already added. " +
                                            "However, the passwords do not match. In order to resolve the issue, from a device this account is already added, " +
                                            "you can remove the device with our public key and try to add account again."
                            }
                            e is StorageError.EmailAlreadyExists -> {
                                error =
                                    "An account with this email already exists. You can find it under Accounts page."
                            }
                            else -> e.handle()
                        }
                    } finally {
                        inProgress = false
                    }
                }
            },
        ) {
            Text("Request Verification")
        }
    }

    error?.let {
        AlertDialog(
            onDismissRequest = { error = null },
            text = { Text(it) },
            confirmButton = { }
        )
    }
}

@Composable
fun ShowPublicKey(email: String, token: String, onAccountAdd: () -> Unit) {
    val scrollState = rememberScrollState()

    var error by remember { mutableStateOf<String?>(null) }
    var publicKey by remember { mutableStateOf<String?>(null) }

    LaunchedEffect(key1 = 1) {
        try {
            publicKey = AccountViewModel.publicKey()
        } catch (e: NoteError) {
            e.handle()
        }
    }

    LaunchedEffect(key1 = 1) {
        try {
            AccountViewModel.waitVerification(token)
            AccountViewModel.addAccount(email)
            onAccountAdd()
        } catch (e: NoteError) {
            when {
                e is MavinoteError.Message && e.message == "ws_failed" -> {
                    error = "Could not wait for verification. Please try again."
                }
                e is MavinoteError.Message && e.message == "ws_timeout" -> {
                    error = "5 minutes waiting is timed out. Please try again."
                }
                else -> e.handle()
            }
        }
    }

    Column(modifier = Modifier.verticalScroll(scrollState)) {
        Text(
            "A verification request is sent to server for $email email address.",
            modifier = Modifier.padding(0.dp, 8.dp)
        )

        Text(
            "In order to complete the progress, on the other device that has already account added, " +
                    "you need to choose Add Device and enter the Public Key displayed below. Please note that Public Key does not contain any line break.",
            modifier = Modifier.padding(0.dp, 8.dp)
        )

        Text(
            "You have 5 min to complete progress",
            modifier = Modifier.padding(0.dp, 16.dp)
        )

        publicKey?.let {
            Text(
                text = "Public Key:",
                modifier = Modifier.padding(0.dp, 0.dp, 0.dp, 12.dp),
            )
            Text(it, fontWeight = FontWeight.Bold)
        }
    }

    error?.let {
        AlertDialog(
            onDismissRequest = { error = null },
            text = { Text(it) },
            confirmButton = { }
        )
    }
}

@Composable
fun SendVerificationCode(navController: NavController) {
    var validationErrors by remember { mutableStateOf<Set<ValidationErrors>>(setOf()) }
    var error by remember { mutableStateOf<String?>(null) }
    var inProgress by remember { mutableStateOf(false) }

    val coroutineScope = rememberCoroutineScope()
    val scrollState = rememberScrollState()

    var email by remember { mutableStateOf("") }

    Column {
        Column(
            modifier = Modifier
                .verticalScroll(scrollState)
                .weight(1f)
        ) {
            Text(
                "Email address is used to identify accounts.",
                modifier = Modifier.padding(0.dp, 8.dp)
            )

            Text(
                "Please enter an email address to create an account for it.",
                modifier = Modifier.padding(0.dp, 16.dp)
            )

            Text(
                text = stringResource(R.string.email),
                modifier = Modifier.padding(0.dp, 0.dp, 0.dp, 12.dp)
            )
            TextField(
                value = email,
                onValueChange = { email = it },
                modifier = Modifier
                    .fillMaxWidth()
                    .padding(0.dp, 0.dp, 0.dp, 12.dp)
            )

            if (validationErrors.contains(ValidationErrors.InvalidEmail)) {
                ErrorText(error = "Please specify a valid email")
            }
        }

        Button(
            modifier = Modifier.fillMaxWidth(),
            enabled = !inProgress,
            onClick = {
                if (inProgress) {
                    return@Button
                }

                val mutableValidationErrors = mutableSetOf<ValidationErrors>()

                if (email.isBlank()) {
                    mutableValidationErrors.add(ValidationErrors.InvalidEmail)
                }


                if (mutableValidationErrors.size != 0) {
                    validationErrors = mutableValidationErrors
                    return@Button
                }

                validationErrors = setOf()
                inProgress = true

                coroutineScope.launch {
                    try {
                        AccountViewModel.sendVerificationCode(email)
                        navController.navigate("verify-code?email=$email")
                    } catch (e: NoteError) {
                        when {
                            e is StorageError.EmailAlreadyExists -> {
                                error =
                                    "An account with this email already exists. You can find it under Accounts page."
                            }
                            e is MavinoteError.Message && e.message == "email_already_used" -> {
                                error =
                                    "This email address is already used for another account. You can add it by choosing Add an Existing Account option."
                            }
                            else -> e.handle()
                        }
                    } finally {
                        inProgress = false
                    }
                }
            },
        ) {
            Text("Send Verification Code")
        }
    }

    error?.let {
        AlertDialog(
            onDismissRequest = { error = null },
            text = { Text(it) },
            confirmButton = { }
        )
    }
}

@Composable
fun VerifyCode(email: String, onVerify: () -> Unit) {
    var validationErrors by remember { mutableStateOf<Set<ValidationErrors>>(setOf()) }
    var error by remember { mutableStateOf<String?>(null) }
    var inProgress by remember { mutableStateOf(false) }

    val coroutineScope = rememberCoroutineScope()
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
                "Please enter verification code to ensure that email belongs to you.",
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

            if (validationErrors.contains(ValidationErrors.InvalidCode)) {
                ErrorText(error = "Please specify the verification code")
            }
        }

        Button(
            modifier = Modifier.fillMaxWidth(),
            enabled = !inProgress,
            onClick = {
                if (inProgress) {
                    return@Button
                }

                val mutableValidationErrors = mutableSetOf<ValidationErrors>()

                if (code.isBlank()) {
                    mutableValidationErrors.add(ValidationErrors.InvalidCode)
                }


                if (mutableValidationErrors.size != 0) {
                    validationErrors = mutableValidationErrors
                    return@Button
                }

                validationErrors = setOf()
                inProgress = true

                coroutineScope.launch {
                    try {
                        AccountViewModel.signUp(email, code)
                        onVerify()
                    } catch (e: NoteError) {
                        when {
                            e is StorageError.EmailAlreadyExists -> {
                                error =
                                    "An account with this email already exists. You can find it under Accounts page."
                            }
                            e is MavinoteError.Message && e.message == "expired_code" -> {
                                error = "5 minutes waiting is timed out. Please try again."
                            }
                            e is MavinoteError.Message && e.message == "invalid_code" -> {
                                error =
                                    "You have entered invalid code. Please check the verification code."
                            }
                            else -> e.handle()
                        }
                    } finally {
                        inProgress = false
                    }
                }
            },
        ) {
            Text("Verify Code")
        }
    }

    error?.let {
        AlertDialog(
            onDismissRequest = { error = null },
            text = { Text(it) },
            confirmButton = { }
        )
    }
}

@Preview(showBackground = true)
@Composable
fun ChooseAccountAddKindPreview() {
    val navController = rememberNavController()

    MavinoteTheme {
        ChooseAccountAddKind(navController)
    }
}

@Preview(showBackground = true)
@Composable
fun EnterAccountInfoPreview() {
    val navController = rememberNavController()

    MavinoteTheme {
        EnterAccountInfo(navController) { }
    }
}

@Preview(showBackground = true)
@Composable
fun ShowPublicKeyPreview() {
    MavinoteTheme {
        ShowPublicKey("hello@email.com", "Token") { }
    }
}

@Preview(showBackground = true)
@Composable
private fun SendVerificationCodePreview() {
    MavinoteTheme {
        SendVerificationCode(rememberNavController())
    }
}

@Preview(showBackground = true)
@Composable
private fun VerifyCodePreview() {
    MavinoteTheme {
        VerifyCode("hello@email.com") { }
    }
}
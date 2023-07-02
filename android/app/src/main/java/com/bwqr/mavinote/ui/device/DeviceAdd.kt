package com.bwqr.mavinote.ui.device

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.AlertDialog
import androidx.compose.material3.Button
import androidx.compose.material3.Text
import androidx.compose.material3.TextField
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.navigation.NavController
import com.bwqr.mavinote.models.MavinoteError
import com.bwqr.mavinote.models.NoteError
import com.bwqr.mavinote.ui.ErrorText
import com.bwqr.mavinote.ui.Title
import com.bwqr.mavinote.viewmodels.AccountViewModel
import kotlinx.coroutines.launch

@Composable
fun DeviceAdd(navController: NavController, accountId: Int) {
    val scope = rememberCoroutineScope()
    var inProgress by remember { mutableStateOf(false) }
    var validationErrors by remember { mutableStateOf(setOf<ValidationErrors>()) }
    var error by remember { mutableStateOf<String?>(null) }

    DeviceAddView(
        inProgress,
        error,
        validationErrors,
        { error = null },
    ) { pubkey ->
        if (inProgress) {
            return@DeviceAddView
        }

        val mutableValidationErrors = mutableSetOf<ValidationErrors>()

        if (pubkey.isBlank()) {
            mutableValidationErrors.add(ValidationErrors.InvalidPubkey)
        }

        if (mutableValidationErrors.size != 0) {
            validationErrors = mutableValidationErrors
            return@DeviceAddView
        }

        validationErrors = setOf()
        inProgress = true

        scope.launch {
            try {
                AccountViewModel.addDevice(accountId, pubkey)
                navController.navigateUp()
            } catch (e: NoteError) {
                when {
                    e is MavinoteError.Message && e.message == "item_not_found" -> {
                        error = "Public Key is not found"
                    }
                    e is MavinoteError.Message && e.message == "device_already_exists" -> {
                        error = "Device with this public key is already added"
                    }
                    e is MavinoteError.Message && e.message == "expired_pubkey" -> {
                        error = "5 minutes waiting is timed out. Please try the steps on new device again."
                    }
                    else -> e.handle()
                }
            } finally {
                inProgress = false
            }
        }
    }
}

private enum class ValidationErrors {
    InvalidPubkey,
}

@Composable
private fun DeviceAddView(
    inProgress: Boolean,
    error: String?,
    validationErrors: Set<ValidationErrors>,
    onDismissError: () -> Unit,
    onDeviceAdd: (fingerprint: String) -> Unit
) {
    val scrollState = rememberScrollState()

    var pubkey by remember { mutableStateOf("") }

    Column(
        verticalArrangement = Arrangement.spacedBy(24.dp),
        modifier = Modifier.padding(16.dp)
    ) {
        Title("Add Device")

        Column {
            Column(modifier = Modifier.verticalScroll(scrollState).weight(1f)) {
                Text(
                    "Cryptographic keys, called Public Key, are used to identify devices.",
                    modifier = Modifier.padding(0.dp, 8.dp)
                )

                Text(
                    "In order to add a new device into this account, you first need to choose Add an Existing Account in Add Account page on new device.",
                    modifier = Modifier.padding(0.dp, 8.dp)
                )

                Text(
                    "Then you need to type the Public Key of the new device below and tap Add Device.",
                    modifier = Modifier.padding(0.dp, 8.dp)
                )

                Text(
                    "Device Public Key",
                    modifier = Modifier.padding(0.dp, 16.dp, 0.dp, 12.dp)
                )
                TextField(
                    value = pubkey,
                    onValueChange = { pubkey = it },
                    modifier = Modifier
                        .fillMaxWidth()
                        .padding(0.dp, 0.dp, 0.dp, 12.dp)
                )

                if (validationErrors.contains(ValidationErrors.InvalidPubkey)) {
                    ErrorText(error = "Please specify a valid Public Key")
                }
            }

            Button(
                modifier = Modifier.fillMaxWidth(),
                enabled = !inProgress,
                onClick = { onDeviceAdd(pubkey) },
            ) {
                Text("Add Device")
            }
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
fun DeviceAddPreview() {
    val inProgress = false
    val error: String? = null
    val validationErrors = setOf<ValidationErrors>()

    DeviceAddView(inProgress, error, validationErrors, { }) { }
}
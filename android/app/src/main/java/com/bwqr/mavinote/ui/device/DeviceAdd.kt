package com.bwqr.mavinote.ui.device

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
import com.bwqr.mavinote.R
import com.bwqr.mavinote.models.MavinoteError
import com.bwqr.mavinote.models.NoteError
import com.bwqr.mavinote.ui.ErrorText
import com.bwqr.mavinote.ui.Title
import com.bwqr.mavinote.viewmodels.NoteViewModel
import kotlinx.coroutines.launch

@Composable
fun DeviceAdd(navController: NavController, accountId: Int) {
    val scope = rememberCoroutineScope()
    var inProgress by remember { mutableStateOf(false) }
    var error by remember { mutableStateOf<String?>(null) }

    DeviceAddView(error) { fingerprint ->
        if (inProgress) {
            return@DeviceAddView
        }

        error = null

        if (fingerprint.isBlank()) {
            error = "Please type the device fingerprint"
            return@DeviceAddView
        }

        inProgress = true

        scope.launch {
            try {
                NoteViewModel.addDevice(accountId, fingerprint)
                navController.navigateUp()
            } catch (e: NoteError) {
                if (e is MavinoteError.Message && e.message == "item_not_found") {
                    error = "Fingerprint not found"
                } else {
                    e.handle()
                }
            } finally {
                inProgress = false
            }
        }
    }
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun DeviceAddView(error: String?, onDeviceAdd: (fingerprint: String) -> Unit) {
    val scrollState = rememberScrollState()

    var fingerprint by remember { mutableStateOf("") }

    Column(modifier = Modifier
        .padding(12.dp)
        .verticalScroll(scrollState)) {
        Title(
            stringResource(R.string.add_device),
            modifier = Modifier.padding(0.dp, 0.dp, 0.dp, 32.dp)
        )

        Text(
            text = "Device Fingerprint",
            modifier = Modifier.padding(0.dp, 0.dp, 0.dp, 12.dp)
        )
        TextField(
            value = fingerprint,
            onValueChange = { fingerprint = it },
            modifier = Modifier.fillMaxWidth()
        )
        
        ErrorText(error)

        Button(
            onClick = { onDeviceAdd(fingerprint) },
            modifier = Modifier.fillMaxWidth()
        ) {
            Text(text = "Add Device")
        }
    }
}

@Preview(showBackground = true)
@Composable
fun DeviceAddPreview() {
    val error: String? = null

    DeviceAddView(error) { }
}
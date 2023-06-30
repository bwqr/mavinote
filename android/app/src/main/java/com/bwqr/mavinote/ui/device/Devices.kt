package com.bwqr.mavinote.ui.device

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.navigation.NavController
import com.bwqr.mavinote.models.Account
import com.bwqr.mavinote.models.AccountKind
import com.bwqr.mavinote.models.Device
import com.bwqr.mavinote.models.NoteError
import com.bwqr.mavinote.ui.Title
import com.bwqr.mavinote.viewmodels.AccountViewModel
import kotlinx.coroutines.launch

@Composable
fun Devices(accountId: Int) {
    val coroutine = rememberCoroutineScope()

    var inProgress by remember { mutableStateOf(false) }
    var account by remember { mutableStateOf<Account?>(null) }
    var devices by remember { mutableStateOf(listOf<Device>()) }

    LaunchedEffect(key1 = 0) {
        launch {
            try {
                account = AccountViewModel.account(accountId)
            } catch (e: NoteError) {
                e.handle()
            }
        }
        launch {
            try {
                devices = AccountViewModel.devices(accountId)
            } catch (e: NoteError) {
                e.handle()
            }
        }

    }


    account?.let {
        DevicesView(it, devices) { deviceId ->
            if (inProgress) {
                return@DevicesView
            }

            inProgress = true

            coroutine.launch {
                try {
                    AccountViewModel.deleteDevice(accountId, deviceId)
                    devices = devices.filter { d -> d.id != deviceId }
                } catch (e: NoteError) {
                    e.handle()
                } finally {
                    inProgress = false
                }
            }
        }
    }
}

@Composable
fun DevicesView(
    account: Account,
    devices: List<Device>,
    onDeleteDevice: (deviceId: Int) -> Unit,
) {
    var deviceToDelete by remember { mutableStateOf<Device?>(null) }

    Column(modifier = Modifier.padding(12.dp)) {
        Row {
            Title(account.name, modifier = Modifier.weight(1f))
        }

        Text(
            text = "Devices",
            modifier = Modifier.padding(0.dp, 0.dp, 0.dp, 12.dp)
        )

        if (devices.isEmpty()) {
            Text("There is no other device for this account. You can add new devices.")
        }

        LazyColumn {
            items(devices.mapIndexed { index, device ->
                Pair(
                    index,
                    device
                )
            }) { (index, device) ->
                ListItem(
                    headlineContent = { Text(device.pubkey) },
                    supportingContent = { Text("Device is added at ${device.createdAt}") },
                    trailingContent = {
                        Icon(
                            Icons.Filled.Delete,
                            contentDescription = null,
                            tint = MaterialTheme.colorScheme.error,
                            modifier = Modifier.clickable { deviceToDelete = device }
                        )
                    }
                )

                if (index != devices.size - 1) {
                    Divider()
                }
            }
        }
    }

    deviceToDelete?.let {
        AlertDialog(
            onDismissRequest = { deviceToDelete = null },
            text = {
                Column {
                    Text("Are you sure about deleting device?", modifier = Modifier.padding(0.dp, 8.dp))

                    Text("Deleted device will not be able to access the account's notes and folders anymore.", modifier = Modifier.padding(0.dp, 8.dp))

                    Text("Deleting a device will also cause any non synced notes and folders on the device to be lost.", modifier = Modifier.padding(0.dp, 8.dp))
                }
            },
            confirmButton = {
                Button(
                    modifier = Modifier.fillMaxWidth(),
                    colors = ButtonDefaults.buttonColors(containerColor = MaterialTheme.colorScheme.error),
                    onClick = {
                        onDeleteDevice(it.id)
                        deviceToDelete = null
                    },
                ) {
                    Text("Delete Device")
                }
            }
        )
    }
}

@Composable
fun DevicesFab(navController: NavController, accountId: Int) {
    ExtendedFloatingActionButton(
        text = { Text("Device") },
        icon = { Icon(Icons.Filled.Add, contentDescription = null) },
        onClick = { navController.navigate("device-add?accountId=$accountId") }
    )
}

@Preview(showBackground = true)
@Composable
fun DevicesPreview() {
    val account = Account(1, "My Account", AccountKind.Mavinote)
    val devices = listOf(
        Device(1, 1, "Device pubkey", "2022 12 12"),
        Device(1, 1, "Device pubkey", "2022 12 12"),
        Device(1, 1, "Device pubkey", "2022 12 12")
    )

    DevicesView(account, devices) { }
}
package com.bwqr.mavinote.ui.device

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Add
import androidx.compose.material3.Divider
import androidx.compose.material3.ExtendedFloatingActionButton
import androidx.compose.material3.Icon
import androidx.compose.material3.Text
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
        DevicesView(it, devices)
    }
}

@Composable
fun DevicesView(account: Account, devices: List<Device>) {
    Column(modifier = Modifier.padding(12.dp)) {
        Row {
            Title(account.name, modifier = Modifier.weight(1f))
        }

        Text(
            text = "Devices",
            modifier = Modifier.padding(0.dp, 0.dp, 0.dp, 12.dp)
        )


        LazyColumn {
            items(devices.mapIndexed { index, device ->
                Pair(
                    index,
                    device
                )
            }) { (index, device) ->
                Text(
                    device.pubkey,
                    modifier = Modifier
                        .fillMaxWidth()
                        .padding(8.dp, 20.dp)
                )

                if (index != devices.size - 1) {
                    Divider()
                }
            }
        }
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
        Device(1, 1, "Device pubkey"),
        Device(1, 1, "Device pubkey"),
        Device(1, 1, "Device pubkey")
    )

    DevicesView(account, devices)
}
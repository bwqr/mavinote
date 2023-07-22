package com.bwqr.mavinote.ui.misc

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.KeyboardArrowRight
import androidx.compose.material3.Divider
import androidx.compose.material3.Icon
import androidx.compose.material3.ListItem
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import androidx.navigation.NavController
import com.bwqr.mavinote.ui.Screen
import com.bwqr.mavinote.ui.util.Title

@Composable
fun Navigations(navController: NavController) {
    val scrollState = rememberScrollState()

    Column(
        verticalArrangement = Arrangement.spacedBy(24.dp),
        modifier = Modifier
            .padding(16.dp)
            .verticalScroll(scrollState)
    ) {
        Row {
            Title("Mavinote", modifier = Modifier.weight(1f))
        }

        Column {
            ListItem(
                headlineContent = { Text("Accounts") },
                trailingContent = {
                    Icon(
                        Icons.Filled.KeyboardArrowRight,
                        contentDescription = null
                    )
                },
                modifier = Modifier.clickable { navController.navigate(Screen.Account.Accounts.route) }
            )

            Divider()

            ListItem(
                headlineContent = { Text("Welcome Page") },
                trailingContent = {
                    Icon(
                        Icons.Filled.KeyboardArrowRight,
                        contentDescription = null
                    )
                },
                modifier = Modifier.clickable { navController.navigate(Screen.Misc.Welcome.route) }
            )

            Divider()

            ListItem(
                headlineContent = { Text("Help") },
                trailingContent = {
                    Icon(
                        Icons.Filled.KeyboardArrowRight,
                        contentDescription = null
                    )
                },
                modifier = Modifier.clickable { navController.navigate(Screen.Misc.Help.route) }
            )

            Divider()
        }
    }
}
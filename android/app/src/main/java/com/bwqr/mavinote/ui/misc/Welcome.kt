package com.bwqr.mavinote.ui.misc

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.Button
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalUriHandler
import androidx.compose.ui.text.style.TextDecoration
import androidx.compose.ui.tooling.preview.Preview
import androidx.navigation.NavController
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import com.bwqr.mavinote.models.NoteError
import com.bwqr.mavinote.ui.Screen
import com.bwqr.mavinote.ui.theme.Spacing
import com.bwqr.mavinote.ui.theme.Typography
import com.bwqr.mavinote.viewmodels.AccountViewModel
import kotlinx.coroutines.launch

sealed class WelcomeScreen(route: String) : Screen(route) {
    object Mavinote : WelcomeScreen("welcome/mavinote")
    object AccountAndDevice : WelcomeScreen("welcome/account-device")
    object FolderAndNote : WelcomeScreen("welcome/folder-note")
}

@Composable
fun Welcome(navController: NavController) {
    val welcomeNavController = rememberNavController()

    Column {
        NavHost(
            welcomeNavController,
            startDestination = WelcomeScreen.Mavinote.route,
        ) {
            composable(WelcomeScreen.Mavinote.route) {
                WelcomeMavinote(navController = welcomeNavController)
            }

            composable(WelcomeScreen.AccountAndDevice.route) {
                AccountAndDeviceManagement(welcomeNavController)
            }

            composable(WelcomeScreen.FolderAndNote.route) {
                FolderAndNoteManagement(navController)
            }
        }
    }
}

@Composable
fun WelcomeMavinote(navController: NavController) {
    val scrollState = rememberScrollState()
    val localUriHandler = LocalUriHandler.current

    Column(verticalArrangement = Arrangement.spacedBy(Spacing.SectionSpacing)) {
        Text(
            "Welcome to Mavinote",
            style = Typography.headlineLarge,
            modifier = Modifier.padding(
                top = Spacing.ScreenPadding,
                start = Spacing.ScreenPadding,
                end = Spacing.ScreenPadding
            )
        )

        Column(
            verticalArrangement = Arrangement.spacedBy(Spacing.ColumnSpacing),
            modifier = Modifier
                .verticalScroll(scrollState)
                .padding(horizontal = Spacing.ScreenPadding)
                .weight(1f)
        ) {
            Text("Mavinote is a simple, open-source, secure, and multi device note taking application.")

            Text("You can take notes that reside only on your device, or create a Mavinote account to store them in the cloud.")

            Text(
                "You can access your notes from other devices by adding your existing account into them." +
                        " All the notes stored in the cloud are encrypted and only readable by your devices."
            )

            Text(
                "Please note that Mavinote is in beta stage, meaning, it is not fully stable yet and subject to frequent changes." +
                        " Any suggestions for improvement are welcome. You can state your suggestions in the Mavinote repository as an issue or discussion." +
                        " You can find the repository in the link below."
            )

            Text(
                "https://github.com/bwqr/mavinote",
                textDecoration = TextDecoration.Underline,
                modifier = Modifier.clickable { localUriHandler.openUri("https://github.com/bwqr/mavinote") }
            )
        }

        Button(
            modifier = Modifier
                .fillMaxWidth()
                .padding(
                    start = Spacing.ScreenPadding,
                    end = Spacing.ScreenPadding,
                    bottom = Spacing.ScreenPadding
                ),
            onClick = { navController.navigate(WelcomeScreen.AccountAndDevice.route) }
        ) {
            Text("Learn About Accounts and Devices")
        }
    }
}

@Composable
fun AccountAndDeviceManagement(navController: NavController) {
    val scrollState = rememberScrollState()

    Column(verticalArrangement = Arrangement.spacedBy(Spacing.SectionSpacing)) {
        Text(
            "Manage Accounts and Devices",
            style = Typography.headlineLarge,
            modifier = Modifier.padding(
                top = Spacing.ScreenPadding,
                start = Spacing.ScreenPadding,
                end = Spacing.ScreenPadding
            )
        )

        Column(
            verticalArrangement = Arrangement.spacedBy(Spacing.ColumnSpacing),
            modifier = Modifier
                .verticalScroll(scrollState)
                .padding(horizontal = Spacing.ScreenPadding)
                .weight(1f)
        ) {

            Text(
                "Mavinote has an hierarchy defined by accounts, devices, folders and notes." +
                        " An account is an entity that stores the folders and notes under itself." +
                        " Mavinote creates an account by default, called Local, when you launch it for the first time." +
                        " This account enables you to take notes that will reside only on your device."
            )

            Text(
                "A device is responsible for managing the accounts." +
                        " In addition to Local account, you can add multiple accounts, called Mavinote, to your devices." +
                        " These Mavinote accounts enable you to take notes that will be synchronized between your other devices." +
                        " In order to synchronize the notes, you need to add same Mavinote account to your devices."
            )
        }

        Button(
            modifier = Modifier
                .fillMaxWidth()
                .padding(
                    start = Spacing.ScreenPadding,
                    end = Spacing.ScreenPadding,
                    bottom = Spacing.ScreenPadding
                ),
            onClick = { navController.navigate(WelcomeScreen.FolderAndNote.route) }
        ) {
            Text("Learn About Folders and Notes")
        }
    }
}

@Composable
fun FolderAndNoteManagement(mainNavController: NavController) {
    val coroutineScope = rememberCoroutineScope()
    val scrollState = rememberScrollState()

    Column(verticalArrangement = Arrangement.spacedBy(Spacing.SectionSpacing)) {
        Text(
            "Create Folders and Notes",
            style = Typography.headlineLarge,
            modifier = Modifier.padding(
                top = Spacing.ScreenPadding,
                start = Spacing.ScreenPadding,
                end = Spacing.ScreenPadding
            )
        )

        Column(
            verticalArrangement = Arrangement.spacedBy(Spacing.ColumnSpacing),
            modifier = Modifier
                .verticalScroll(scrollState)
                .padding(horizontal = Spacing.ScreenPadding)
                .weight(1f)
        ) {
            Text(
                "In Mavinote application, you can create folders and put notes into them." +
                        " In order to create a note, you first need to create a folder." +
                        " Folders can be created by specifying a name."
            )

            Text(
                "If you have more than one account, you also need to choose an account while creating a folder." +
                        " You can create notes after navigating to any folder." +
                        " Right now, Mavinote application only supports taking plain text notes." +
                        " Note editing will be improved with upcoming releases."
            )

            Text("Now you are ready to dive into Mavinote application. Go ahead and start taking notes.")
        }

        Button(
            modifier = Modifier
                .fillMaxWidth()
                .padding(
                    start = Spacing.ScreenPadding,
                    end = Spacing.ScreenPadding,
                    bottom = Spacing.ScreenPadding
                ),
            onClick = {
                coroutineScope.launch {
                    try {
                        AccountViewModel.updateWelcomeShown(true)

                        mainNavController.navigate(Screen.Note.Folders.route) {
                            popUpTo(0)
                        }
                    } catch (e: NoteError) {
                        e.handle()
                    }
                }

            }
        ) {
            Text("Start Using Mavinote")
        }
    }
}

@Preview(showBackground = true)
@Composable
fun WelcomeMavinotePreview() {
    val navController = rememberNavController()
    WelcomeMavinote(navController)
}

@Preview(showBackground = true)
@Composable
fun AccountAndDeviceManagementPreview() {
    val navController = rememberNavController()
    AccountAndDeviceManagement(navController)
}

@Preview(showBackground = true)
@Composable
fun FolderAndNoteManagementPreview() {
    val navController = rememberNavController()
    FolderAndNoteManagement(navController)
}
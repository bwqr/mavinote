package com.bwqr.mavinote.ui

import androidx.compose.foundation.layout.padding
import androidx.compose.material.Scaffold
import androidx.compose.material.rememberScaffoldState
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.ui.Modifier
import androidx.navigation.NavType
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.currentBackStackEntryAsState
import androidx.navigation.compose.rememberNavController
import androidx.navigation.navArgument
import com.bwqr.mavinote.Bus
import com.bwqr.mavinote.BusEvent
import com.bwqr.mavinote.models.NoteError
import com.bwqr.mavinote.ui.account.*
import com.bwqr.mavinote.ui.device.DeviceAdd
import com.bwqr.mavinote.ui.note.*
import com.bwqr.mavinote.viewmodels.NoteViewModel
import kotlinx.coroutines.launch

open class Screen(val route: String) {
    sealed class Note(route: String) : Screen(route) {
        object Folders : Screen.Note("folders")
        object FolderCreate : Screen.Note("folder-create")
        object Notes : Screen.Note("notes/{folderId}")
        object Note : Screen.Note("note?noteId={noteId}&folderId={folderId}")
    }

    sealed class Account(route: String) : Screen(route) {
        object Accounts : Screen.Account("accounts")
        object AccountAdd : Screen.Account("account-add")
        object Account : Screen.Account("account/{accountId}")
    }

    sealed class Device(route: String) : Screen(route) {
        object DeviceAdd : Screen.Device("device-add?accountId={accountId}")
    }
}

@Composable
fun BackgroundFeatures() {
    val scaffoldState = rememberScaffoldState()
    val navController = rememberNavController()
    val backstackEntry = navController.currentBackStackEntryAsState()

    LaunchedEffect(key1 = 1) {
        launch {
            while (true) {
                when (val event = Bus.listen()) {
                    is BusEvent.DisplayNoInternetWarning -> scaffoldState.snackbarHostState.showSnackbar(
                        "No internet connection"
                    )
                    is BusEvent.UnhandledError -> scaffoldState.snackbarHostState.showSnackbar(event.error)
                }
            }
        }

        try {
            NoteViewModel.sync()
        } catch (e: NoteError) {
            e.handle()
        }
    }

    Scaffold(
        scaffoldState = scaffoldState,
        floatingActionButton = {
            when (backstackEntry.value?.destination?.route) {
                Screen.Note.Folders.route -> FoldersFab(navController)
                Screen.Note.Notes.route -> NotesFab(
                    navController,
                    navController.currentBackStackEntry?.arguments?.getInt("folderId")!!
                )
                Screen.Account.Accounts.route -> AccountsFab(navController)
                Screen.Account.Account.route -> AccountFab(
                    navController,
                    navController.currentBackStackEntry?.arguments?.getInt("accountId")!!
                )
            }
        },
    ) {

        NavHost(
            navController,
            startDestination = Screen.Note.Folders.route,
            modifier = Modifier.padding(it)
        ) {
            composable(Screen.Account.Accounts.route) { Accounts(navController) }
            composable(
                Screen.Account.Account.route,
                arguments = listOf(navArgument("accountId") { type = NavType.IntType })
            ) { backstackEntry ->
                Account(
                    navController,
                    backstackEntry.arguments?.getInt("accountId")!!
                )
            }
            composable(Screen.Account.AccountAdd.route) { AccountAdd(navController) }
            composable(
                Screen.Device.DeviceAdd.route,
                arguments = listOf(navArgument("accountId") { type = NavType.IntType })
            ) { backstackEntry ->
                DeviceAdd(
                    navController,
                    backstackEntry.arguments?.getInt("accountId")!!
                )
            }

            composable(Screen.Note.Folders.route) { Folders(navController) }
            composable(Screen.Note.FolderCreate.route) { FolderCreate(navController) }

            composable(
                Screen.Note.Notes.route,
                arguments = listOf(navArgument("folderId") { type = NavType.IntType })
            ) { backStackEntry ->
                Notes(
                    navController,
                    backStackEntry.arguments?.getInt("folderId")!!
                )
            }

            composable(
                Screen.Note.Note.route,
                arguments = listOf(
                    navArgument("folderId") { nullable = true },
                    navArgument("noteId") { nullable = true },
                )
            ) { backStackEntry ->
                Note(
                    navController,
                    backStackEntry.arguments?.getString("folderId")?.toInt(),
                    backStackEntry.arguments?.getString("noteId")?.toInt(),
                )
            }
        }
    }
}
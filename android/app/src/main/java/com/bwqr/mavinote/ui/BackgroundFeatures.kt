package com.bwqr.mavinote.ui

import android.util.Log
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Scaffold
import androidx.compose.material3.SnackbarHost
import androidx.compose.material3.SnackbarHostState
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.navigation.NavType
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.currentBackStackEntryAsState
import androidx.navigation.compose.rememberNavController
import androidx.navigation.navArgument
import com.bwqr.mavinote.Bus
import com.bwqr.mavinote.BusEvent
import com.bwqr.mavinote.models.AccountKind
import com.bwqr.mavinote.models.NoteError
import com.bwqr.mavinote.ui.account.Account
import com.bwqr.mavinote.ui.account.AccountAdd
import com.bwqr.mavinote.ui.account.AccountClose
import com.bwqr.mavinote.ui.account.Accounts
import com.bwqr.mavinote.ui.account.AccountsFab
import com.bwqr.mavinote.ui.device.DeviceAdd
import com.bwqr.mavinote.ui.device.Devices
import com.bwqr.mavinote.ui.device.DevicesFab
import com.bwqr.mavinote.ui.note.FolderCreate
import com.bwqr.mavinote.ui.note.Folders
import com.bwqr.mavinote.ui.note.FoldersFab
import com.bwqr.mavinote.ui.note.Note
import com.bwqr.mavinote.ui.note.Notes
import com.bwqr.mavinote.ui.note.NotesFab
import com.bwqr.mavinote.viewmodels.AccountViewModel
import com.bwqr.mavinote.viewmodels.NoteViewModel
import kotlinx.coroutines.Job
import kotlinx.coroutines.channels.consumeEach
import kotlinx.coroutines.flow.catch
import kotlinx.coroutines.flow.launchIn
import kotlinx.coroutines.flow.map
import kotlinx.coroutines.flow.onCompletion
import kotlinx.coroutines.flow.onEach
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
        object AccountClose : Screen.Account("account-close?accountId={accountId}")
    }

    sealed class Device(route: String) : Screen(route) {
        object Devices : Device("devices?accountId={accountId}")
        object DeviceAdd : Device("device-add?accountId={accountId}")
    }
}

@Composable
fun BackgroundFeatures() {
    val snackbarHostState = remember { SnackbarHostState() }
    val navController = rememberNavController()
    val backstackEntry = navController.currentBackStackEntryAsState()
    var notificationJobs by remember { mutableStateOf(listOf<Job>()) }

    LaunchedEffect(key1 = 1) {
        launch {
            Bus.listen().consumeEach {
                when (it) {
                    is BusEvent.ShowMessage -> snackbarHostState.showSnackbar(it.message)
                }
            }

            Log.e("BackgroundFeatures", "Bus listening is stopped")
        }

        AccountViewModel
            .accounts()
            .map { it.filter { acc -> acc.kind == AccountKind.Mavinote } }
            .onEach { accounts ->
                notificationJobs.forEach { it.cancel() }

                notificationJobs = accounts.map { account ->
                    AccountViewModel
                        .listenNotifications(account.id)
                        .onEach {
                            Log.i("BackgroundFeatures", "Received notification for account ${account.id}, notification $it")
                        }
                        .catch {
                            Log.e(
                                "BackgroundFeatures",
                                "Failure for account ${account.id}, cause ${it.cause}"
                            )
                        }
                        .onCompletion {
                            Log.d(
                                "BackgroundFeatures",
                                "Job is finished for account ${account.id}"
                            )
                        }
                        .launchIn(this)
                }
            }
            .launchIn(this)

        try {
            NoteViewModel.sync()
        } catch (e: NoteError) {
            e.handle()
        }
    }

    Scaffold(
        snackbarHost = { SnackbarHost(snackbarHostState) },
        floatingActionButton = {
            when (backstackEntry.value?.destination?.route) {
                Screen.Note.Folders.route -> FoldersFab(navController)
                Screen.Note.Notes.route -> NotesFab(
                    navController,
                    navController.currentBackStackEntry?.arguments?.getInt("folderId")!!
                )
                Screen.Account.Accounts.route -> AccountsFab(navController)
                Screen.Device.Devices.route -> DevicesFab(
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
                Screen.Account.AccountClose.route,
                arguments = listOf(navArgument("accountId") { type = NavType.IntType })
            ) { backstackEntry ->
                AccountClose(
                    navController,
                    backstackEntry.arguments?.getInt("accountId")!!
                )
            }

            composable(
                Screen.Device.Devices.route,
                arguments = listOf(navArgument("accountId") { type = NavType.IntType })
            ) { backstackEntry ->
                 Devices(backstackEntry.arguments?.getInt("accountId")!!)
            }
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
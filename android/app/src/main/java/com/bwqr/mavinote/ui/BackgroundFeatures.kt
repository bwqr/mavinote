package com.bwqr.mavinote.ui

import androidx.compose.foundation.layout.padding
import androidx.compose.material.Scaffold
import androidx.compose.material.rememberScaffoldState
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.mutableStateMapOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.navigation.NavType
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.currentBackStackEntryAsState
import androidx.navigation.compose.rememberNavController
import androidx.navigation.navArgument
import com.bwqr.mavinote.Bus
import com.bwqr.mavinote.BusEvent
import com.bwqr.mavinote.models.Error
import com.bwqr.mavinote.ui.note.*
import com.bwqr.mavinote.viewmodels.NoteViewModel
import kotlinx.coroutines.launch

sealed class NoteScreens(val route: String) {
    object Accounts : NoteScreens("accounts")
    object AccountAdd : NoteScreens("account-add")
    object Account : NoteScreens("account/{accountId}")
    object Folders : NoteScreens("folders")
    object FolderCreate : NoteScreens("folder-create")
    object Notes : NoteScreens("notes/{folderId}")
    object Note : NoteScreens("note?noteId={noteId}&folderId={folderId}")
}

@Composable
fun BackgroundFeatures() {
    val scaffoldState = rememberScaffoldState()
    val navController = rememberNavController()
    val backstackEntry = navController.currentBackStackEntryAsState()

    val accountsToAuthorize = remember { mutableStateMapOf<Int, Unit>() }


    LaunchedEffect(key1 = 1) {
        launch {
            while (true) {
                when (val event = Bus.listen()) {
                    is BusEvent.DisplayNoInternetWarning -> scaffoldState.snackbarHostState.showSnackbar(
                        "No internet connection"
                    )
                    is BusEvent.RequireAuthorization -> accountsToAuthorize[event.accountId] = Unit
                }
            }
        }

        try {
            NoteViewModel.sync()
        } catch (e: Error) {
            e.handle()
        }
    }

    Scaffold(
        scaffoldState = scaffoldState,
        floatingActionButton = {
            when (backstackEntry.value?.destination?.route) {
                NoteScreens.Folders.route -> FolderFab(navController)
                NoteScreens.Notes.route -> NotesFab(
                    navController,
                    navController.currentBackStackEntry?.arguments?.getInt("folderId")!!
                )
                NoteScreens.Accounts.route -> AccountsFab(navController)
            }
        },
    ) {
        accountsToAuthorize.firstNotNullOfOrNull { accountId -> accountId.key }?.let { accountId ->
            AccountAuthorize(accountId) { accountsToAuthorize.remove(accountId) }
        }

        NavHost(
            navController,
            startDestination = NoteScreens.Folders.route,
            modifier = Modifier.padding(it)
        ) {
            composable(NoteScreens.Accounts.route) { Accounts(navController) }
            composable(
                NoteScreens.Account.route,
                arguments = listOf(navArgument("accountId") { type = NavType.IntType })
            ) { backstackEntry ->
                Account(
                    navController,
                    backstackEntry.arguments?.getInt("accountId")!!
                )
            }
            composable(NoteScreens.AccountAdd.route) { AccountAdd(navController) }

            composable(NoteScreens.Folders.route) { Folders(navController) }
            composable(NoteScreens.FolderCreate.route) { FolderCreate(navController) }

            composable(
                NoteScreens.Notes.route,
                arguments = listOf(navArgument("folderId") { type = NavType.IntType })
            ) { backStackEntry ->
                Notes(
                    navController,
                    backStackEntry.arguments?.getInt("folderId")!!
                )
            }

            composable(
                NoteScreens.Note.route,
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
package com.bwqr.mavinote.ui

import androidx.compose.foundation.layout.padding
import androidx.compose.material.Scaffold
import androidx.compose.material.rememberScaffoldState
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.navigation.NavController
import androidx.navigation.NavType
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.currentBackStackEntryAsState
import androidx.navigation.compose.rememberNavController
import androidx.navigation.navArgument
import com.bwqr.mavinote.models.ReaxException
import com.bwqr.mavinote.ui.note.*
import com.bwqr.mavinote.viewmodels.Bus
import com.bwqr.mavinote.viewmodels.BusEvent
import com.bwqr.mavinote.viewmodels.NoteViewModel
import kotlinx.coroutines.flow.launchIn
import kotlinx.coroutines.flow.onEach
import kotlinx.coroutines.launch

sealed class NoteScreens(val route: String) {
    object Accounts : NoteScreens("accounts")
    object AccountAdd : NoteScreens("account-add")
    object Folders : NoteScreens("folders")
    object FolderAdd : NoteScreens("folder-add")
    object Notes : NoteScreens("notes/{folderId}")
    object Note : NoteScreens("note?noteId={noteId}&folderId={folderId}")
}

@Composable
fun BackgroundFeatures(mainNavController: NavController) {
    val scaffoldState = rememberScaffoldState()
    val navController = rememberNavController()
    val backstackEntry = navController.currentBackStackEntryAsState()

    var syncing by remember { mutableStateOf(false) }

    LaunchedEffect(key1 = 1) {
        launch {
            while (true) {
                when (Bus.listen()) {
                    BusEvent.DisplayNoInternetWarning -> scaffoldState.snackbarHostState.showSnackbar(
                        "No internet connection"
                    )
                    BusEvent.RequireAuthorization -> scaffoldState.snackbarHostState.showSnackbar(
                        "Mavinote account requires authorization"
                    )
                }
            }
        }

        NoteViewModel()
            .activeSyncs()
            .onEach { syncing = it > 0 }
            .launchIn(this)

        try {
            NoteViewModel().sync()
        } catch (e: ReaxException) {
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
            }
        },
    ) {
        NavHost(
            navController,
            startDestination = NoteScreens.Folders.route,
            modifier = Modifier.padding(it)
        ) {
            composable(NoteScreens.Accounts.route) { Accounts(navController) }
            composable(NoteScreens.AccountAdd.route) { AccountAdd(navController) }

            composable(NoteScreens.Folders.route) { Folders(navController) }
            composable(NoteScreens.FolderAdd.route) { FolderAdd(navController) }

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
                    backStackEntry.arguments?.getString("folderId")?.toInt(),
                    backStackEntry.arguments?.getString("noteId")?.toInt(),
                )
            }
        }
    }
}
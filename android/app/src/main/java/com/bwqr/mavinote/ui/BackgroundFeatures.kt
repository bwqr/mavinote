package com.bwqr.mavinote.ui

import androidx.compose.material.Scaffold
import androidx.compose.material.Text
import androidx.compose.material.rememberScaffoldState
import androidx.compose.runtime.*
import androidx.navigation.*
import androidx.navigation.compose.*
import com.bwqr.mavinote.MainScreens
import com.bwqr.mavinote.models.ReaxException
import com.bwqr.mavinote.viewmodels.Bus
import com.bwqr.mavinote.viewmodels.BusEvent
import com.bwqr.mavinote.viewmodels.NoteViewModel
import com.bwqr.mavinote.ui.note.*
import kotlinx.coroutines.flow.launchIn
import kotlinx.coroutines.flow.onEach
import kotlinx.coroutines.launch

sealed class NoteScreens(val route: String) {
    object Folders : NoteScreens("folders")
    object FolderAdd : NoteScreens("folder-add")
    object Notes : NoteScreens("notes/{folderId}")
    object Note : NoteScreens("note/{noteId}")
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
                    BusEvent.RequireAuthorization -> mainNavController.navigate(MainScreens.Login.route)
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
                    navController.currentBackStackEntry?.arguments?.getInt("folderId") ?: 0
                )
            }
        },
        bottomBar = {
            Text(text = "Syncing $syncing")
        }
    ) {
        NavHost(navController, startDestination = NoteScreens.Folders.route) {
            composable(NoteScreens.Folders.route) { Folders(navController) }
            composable(NoteScreens.FolderAdd.route) { FolderAdd(navController) }

            composable(
                NoteScreens.Notes.route,
                arguments = listOf(navArgument("folderId") { type = NavType.IntType })
            ) { backStackEntry ->
                Notes(
                    navController,
                    backStackEntry.arguments?.getInt("folderId") ?: 0
                )
            }

            composable(
                NoteScreens.Note.route,
                arguments = listOf(navArgument("noteId") { type = NavType.IntType })
            ) { backStackEntry ->
                Note(
                    backStackEntry.arguments?.getInt("noteId") ?: 0
                )
            }
        }
    }
}
package com.bwqr.mavinote

import android.os.Bundle
import android.util.Log
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import androidx.navigation.NavType
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.currentBackStackEntryAsState
import androidx.navigation.compose.rememberNavController
import androidx.navigation.navArgument
import com.bwqr.mavinote.models.ReaxException
import com.bwqr.mavinote.ui.note.*
import com.bwqr.mavinote.ui.auth.Login
import com.bwqr.mavinote.ui.theme.MaviNoteTheme
import com.bwqr.mavinote.viewmodels.Bus
import com.bwqr.mavinote.viewmodels.BusEvent
import com.bwqr.mavinote.viewmodels.NoteViewModel
import com.bwqr.mavinote.viewmodels.Runtime
import kotlinx.coroutines.flow.launchIn
import kotlinx.coroutines.flow.onEach

sealed class Screen(val route: String) {
    object Login : Screen("login")
    object Folders : Screen("folders")
    object FolderAdd : Screen("folder-add")
    object Notes : Screen("notes/{folderId}")
    object Note : Screen("note/{noteId}")
}

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        Runtime.initialize(applicationContext.filesDir.absolutePath)

        setContent {
            MaviNoteTheme {
                // A surface container using the 'background' color from the theme
                Surface(
                    modifier = Modifier.fillMaxSize(),
                    color = MaterialTheme.colors.background
                ) {
                    MainScreen()
                }
            }
        }
    }
}

@Composable
fun MainScreen() {
    val navController = rememberNavController()
    val backstackEntry = navController.currentBackStackEntryAsState()
    val scaffoldState = rememberScaffoldState()
    var syncing by remember {
        mutableStateOf(false)
    }

    LaunchedEffect(key1 = 1) {
        NoteViewModel().activeSyncs().onEach {
            syncing = it > 0
        }.launchIn(this)

        try {
            NoteViewModel().sync()
        } catch (e: ReaxException) {
            e.handle()
        }

        while (true) {
            when (Bus.listen()) {
                BusEvent.DisplayNoInternetWarning -> scaffoldState.snackbarHostState.showSnackbar(
                    "Internet yok la"
                )
                BusEvent.RequireAuthorization -> navController.navigate(Screen.Login.route)
            }
        }
    }

    Scaffold(
        scaffoldState = scaffoldState,
        floatingActionButton = {
            when (backstackEntry.value?.destination?.route) {
                Screen.Folders.route -> FolderFab(navController)
                Screen.Notes.route -> NotesFab(
                    navController,
                    navController.currentBackStackEntry?.arguments?.getInt("folderId") ?: 0
                )
            }
        },
        bottomBar = {
            Text(text = "Syncing $syncing")
        }
    ) {
        NavHost(navController = navController, startDestination = Screen.Folders.route) {
            composable(Screen.Login.route) { Login(navController) }
            composable(Screen.Folders.route) { Folders(navController) }
            composable(Screen.FolderAdd.route) { FolderAdd(navController) }
            composable(
                Screen.Notes.route,
                arguments = listOf(navArgument("folderId") { type = NavType.IntType })
            ) { backStackEntry ->
                Notes(
                    navController,
                    backStackEntry.arguments?.getInt("folderId") ?: 0
                )
            }
            composable(
                Screen.Note.route,
                arguments = listOf(navArgument("noteId") { type = NavType.IntType })
            ) { backStackEntry ->
                Note(
                    backStackEntry.arguments?.getInt("noteId") ?: 0
                )
            }
        }
    }
}

@Preview(showBackground = true)
@Composable
fun DefaultPreview() {
    MaviNoteTheme {
        MainScreen()
    }
}
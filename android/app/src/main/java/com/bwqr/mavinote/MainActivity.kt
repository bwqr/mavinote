package com.bwqr.mavinote

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import com.bwqr.mavinote.ui.theme.MaviNoteTheme
import com.bwqr.mavinote.viewmodels.Runtime
import com.bwqr.mavinote.ui.BackgroundFeatures
import com.bwqr.mavinote.ui.auth.Login

sealed class MainScreens(val route: String) {
    object Login : MainScreens("login")
    object BackgroundFeatures : MainScreens("background-features")
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

    NavHost(
        navController = navController,
        startDestination = MainScreens.BackgroundFeatures.route
    ) {
        composable(MainScreens.Login.route) { Login(navController) }

        composable(MainScreens.BackgroundFeatures.route) {
            BackgroundFeatures(navController)
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
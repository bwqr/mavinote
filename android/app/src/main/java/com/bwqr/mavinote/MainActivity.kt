package com.bwqr.mavinote

import android.content.Context
import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import com.bwqr.mavinote.reax.Runtime
import com.bwqr.mavinote.ui.BackgroundFeatures
import com.bwqr.mavinote.ui.theme.MavinoteTheme
import com.bwqr.mavinote.viewmodels.NoteViewModel

var ReaxInitialized = false

fun initReax(ctx: Context) {
    if (ReaxInitialized) {
        return
    }

    ReaxInitialized = true

    System.loadLibrary("reax")
    Runtime.init(ctx.filesDir.absolutePath)
    NoteViewModel.init()
}

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        initReax(applicationContext)

        setContent {
            MavinoteTheme {
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
    BackgroundFeatures()
}

@Preview(showBackground = true)
@Composable
fun DefaultPreview() {
    MavinoteTheme {
        MainScreen()
    }
}
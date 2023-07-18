package com.bwqr.mavinote

import android.content.Context
import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.text.selection.SelectionContainer
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import com.bwqr.mavinote.reax.Either
import com.bwqr.mavinote.reax.Runtime
import com.bwqr.mavinote.ui.BackgroundFeatures
import com.bwqr.mavinote.ui.theme.MavinoteTheme
import com.bwqr.mavinote.ui.theme.Spacing
import com.bwqr.mavinote.viewmodels.NoteViewModel

fun initReax(ctx: Context) {
    when (val either = Runtime.init(ctx.filesDir.absolutePath)) {
        is Either.Success -> {
            if (!either.value) {
                NoteViewModel.init()
            }
        }

        is Either.Failure -> throw Error(either.value)
    }
}

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        try {
            initReax(applicationContext)
        } catch (e: Error) {
            setContent {
                UnrecoverableError(e)
            }

            return
        }

        setContent {
            MainScreen()
        }
    }
}

@Composable
fun MainScreen() {
    MavinoteTheme {
        // A surface container using the 'background' color from the theme
        Surface(
            modifier = Modifier.fillMaxSize(), color = MaterialTheme.colorScheme.background
        ) {
            BackgroundFeatures()
        }
    }
}

@Composable
fun UnrecoverableError(e: Error) {
    val scrollState = rememberScrollState()

    MavinoteTheme {
        Surface(
            modifier = Modifier.fillMaxSize(), color = MaterialTheme.colorScheme.background
        ) {
            SelectionContainer {
                Column(
                    verticalArrangement = Arrangement.spacedBy(Spacing.SectionSpacing),
                    modifier = Modifier
                        .verticalScroll(scrollState)
                        .padding(Spacing.ScreenPadding)
                ) {
                    Spacer(Modifier.weight(1.0f))

                    Text("An unrecoverable error is encountered while initializing the application")
                    Text("Error: ${e.message}")

                    Spacer(Modifier.weight(1.0f))
                }
            }
        }
    }
}

@Preview(showBackground = true)
@Composable
fun DefaultPreview() {
    MavinoteTheme {
        MainScreen()
    }
}
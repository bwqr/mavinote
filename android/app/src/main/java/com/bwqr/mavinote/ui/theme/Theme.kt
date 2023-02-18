package com.bwqr.mavinote.ui.theme

import androidx.compose.foundation.isSystemInDarkTheme
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material.*
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview

private val DarkColorPalette = darkColors(
    primary = Blue200,
    primaryVariant = Blue500,
    secondary = Gray200
)

private val LightColorPalette = lightColors(
    primary = Blue500,
    primaryVariant = Blue700,
    secondary = Gray400

    /* Other default colors to override
    background = Color.White,
    surface = Color.White,
    onPrimary = Color.White,
    onSecondary = Color.Black,
    onBackground = Color.Black,
    onSurface = Color.Black,
    */
)

@Composable
fun MavinoteTheme(darkTheme: Boolean = isSystemInDarkTheme(), content: @Composable () -> Unit) {
    val colors = if (darkTheme) {
        DarkColorPalette
    } else {
        LightColorPalette
    }

    MaterialTheme(
        colors = colors,
        typography = Typography,
        shapes = Shapes,
        content = content
    )
}

@Composable
private fun MavinoteThemePreview() {
    Column {
        Row(verticalAlignment = Alignment.Bottom) {
            Text("Hello", style = Typography.h1)
            Text("h1")
        }
        Row(verticalAlignment = Alignment.Bottom) {
            Text("Hello", style = Typography.h2)
            Text("h2")
        }
        Row(verticalAlignment = Alignment.Bottom) {
            Text("Hello", style = Typography.h3)
            Text("h3")
        }
        Row(verticalAlignment = Alignment.Bottom) {
            Text("Hello", style = Typography.h4)
            Text("h4")
        }
        Row(verticalAlignment = Alignment.Bottom) {
            Text("Hello", style = Typography.h5)
            Text("h5")
        }
        Row(verticalAlignment = Alignment.Bottom) {
            Text("Hello", style = Typography.h6)
            Text("h6")
        }
        Row(verticalAlignment = Alignment.Bottom) {
            Text("Hello", style = Typography.subtitle1)
            Text("subtitle1")
        }
        Row(verticalAlignment = Alignment.Bottom) {
            Text("Hello", style = Typography.body1)
            Text("body1")
        }
        Row(verticalAlignment = Alignment.Bottom) {
            Button(onClick = { }, colors = ButtonDefaults.buttonColors(MaterialTheme.colors.primary)) {
                Text("Hello", style = Typography.button)
            }
            Text("primary")
        }
        Row(verticalAlignment = Alignment.Bottom) {
            Button(onClick = { }, colors = ButtonDefaults.buttonColors(MaterialTheme.colors.primaryVariant)) {
                Text("Hello")
            }
            Text("primaryVariant")
        }
        Row(verticalAlignment = Alignment.Bottom) {
            Button(onClick = { }, colors = ButtonDefaults.buttonColors(MaterialTheme.colors.secondary)) {
                Text("Hello")
            }
            Text("secondary")
        }
    }
}

@Preview
@Composable
fun LightThemePreview() {
    MavinoteTheme {
        // A surface container using the 'background' color from the theme
        Surface(
            modifier = Modifier.fillMaxSize(),
            color = MaterialTheme.colors.background
        ) {
            MavinoteThemePreview()
        }
    }

}

@Preview
@Composable
fun DarkThemePreview() {
    MavinoteTheme(darkTheme = true) {
        // A surface container using the 'background' color from the theme
        Surface(
            modifier = Modifier.fillMaxSize(),
            color = MaterialTheme.colors.background
        ) {
            MavinoteThemePreview()
        }
    }
}
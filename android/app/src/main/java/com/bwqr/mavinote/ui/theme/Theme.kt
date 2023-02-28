package com.bwqr.mavinote.ui.theme

import androidx.compose.foundation.isSystemInDarkTheme
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material3.*
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview

private val DarkColors = darkColorScheme()

private val LightColors = lightColorScheme()

@Composable
fun MavinoteTheme(darkTheme: Boolean = isSystemInDarkTheme(), content: @Composable () -> Unit) {
    val colors = if (darkTheme) {
        DarkColors
    } else {
        LightColors
    }

    MaterialTheme(
        colorScheme = colors,
        typography = Typography,
        shapes = Shapes,
        content = content
    )
}

@Composable
private fun MavinoteThemePreview() {
    Column {
        Row(verticalAlignment = Alignment.Bottom) {
            Text("Hello", style = Typography.titleSmall)
            Text("titleSmall")
        }
        Row(verticalAlignment = Alignment.Bottom) {
            Text("Hello", style = Typography.titleMedium)
            Text("titleMedium")
        }
        Row(verticalAlignment = Alignment.Bottom) {
            Text("Hello", style = Typography.titleLarge)
            Text("titleLarge")
        }
        Row(verticalAlignment = Alignment.Bottom) {
            Text("Hello", style = Typography.labelSmall)
            Text("labelSmall")
        }
        Row(verticalAlignment = Alignment.Bottom) {
            Text("Hello", style = Typography.labelMedium)
            Text("labelMedium")
        }
        Row(verticalAlignment = Alignment.Bottom) {
            Text("Hello", style = Typography.labelLarge)
            Text("labelLarge")
        }
        Row(verticalAlignment = Alignment.Bottom) {
            Text("Hello", style = Typography.bodySmall)
            Text("bodySmall")
        }
        Row(verticalAlignment = Alignment.Bottom) {
            Text("Hello", style = Typography.bodyMedium)
            Text("bodyMedium")
        }
        Row(verticalAlignment = Alignment.Bottom) {
            Text("Hello", style = Typography.bodyLarge)
            Text("bodyLarge")
        }
        Row(verticalAlignment = Alignment.Bottom) {
            Button(
                onClick = { },
                colors = ButtonDefaults.buttonColors(MaterialTheme.colorScheme.primary)
            ) {
                Text("Hello")
            }
            Text("primary")
        }
        Row(verticalAlignment = Alignment.Bottom) {
            Button(
                onClick = { },
                colors = ButtonDefaults.buttonColors(MaterialTheme.colorScheme.secondary)
            ) {
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
            color = MaterialTheme.colorScheme.background
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
            color = MaterialTheme.colorScheme.background
        ) {
            MavinoteThemePreview()
        }
    }
}
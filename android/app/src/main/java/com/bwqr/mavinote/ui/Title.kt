package com.bwqr.mavinote.ui

import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.*

@OptIn(ExperimentalUnitApi::class)
@Composable
fun Title(text: String, modifier: Modifier = Modifier) {
    Text(
        text,
        fontWeight = FontWeight.ExtraBold,
        fontSize = TextUnit(6f, TextUnitType.Em),
        overflow = TextOverflow.Ellipsis,
        modifier = modifier,
        maxLines = 1,
    )
}

@OptIn(ExperimentalUnitApi::class)
@Composable
fun SubTitle(text: String, modifier: Modifier = Modifier) {
    Text(
        text,
        fontWeight = FontWeight.ExtraBold,
        fontSize = TextUnit(4.5f, TextUnitType.Em),
        modifier = modifier
    )
}
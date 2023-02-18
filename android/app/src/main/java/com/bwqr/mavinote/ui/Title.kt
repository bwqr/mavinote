package com.bwqr.mavinote.ui

import androidx.compose.foundation.layout.padding
import androidx.compose.material.LocalTextStyle
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.*
import com.bwqr.mavinote.ui.theme.Typography

@Composable
fun Title(text: String, modifier: Modifier = Modifier) {
    Text(
        text,
        style = Typography.h4,
        overflow = TextOverflow.Ellipsis,
        modifier = modifier,
        maxLines = 1,
    )
}

@Composable
fun ErrorText(error: String?) {
    error?.let {
        Text(
            text = it,
            color = MaterialTheme.colors.error,
            modifier = Modifier.padding(0.dp, 0.dp, 0.dp, 16.dp)
        )
    }
}
package com.bwqr.mavinote.ui.misc

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalUriHandler
import androidx.compose.ui.text.style.TextDecoration
import com.bwqr.mavinote.ui.theme.Spacing
import com.bwqr.mavinote.ui.theme.Typography

@Composable
fun Help() {
    val scrollState = rememberScrollState()
    val localUriHandler = LocalUriHandler.current

    Column(verticalArrangement = Arrangement.spacedBy(Spacing.SectionSpacing)) {
        Text(
            "Help",
            style = Typography.headlineLarge,
            modifier = Modifier.padding(
                top = Spacing.ScreenPadding,
                start = Spacing.ScreenPadding,
                end = Spacing.ScreenPadding
            )
        )

        Column(
            verticalArrangement = Arrangement.spacedBy(Spacing.ColumnSpacing),
            modifier = Modifier
                .verticalScroll(scrollState)
                .padding(horizontal = Spacing.ScreenPadding)
                .weight(1f)
        ) {
            Text(
                "Mavinote is in beta stage, meaning, it is not fully stable yet and subject to frequent changes." +
                        " Right now, there is not much knowledge written in somewhere except the repository and the application themselves."
            )

            Text(
                "For any kind of help you need, please take a look into the repository issues or discussions." +
                        " If you are not able to find a similar topic to yours, please create a new one." +
                        " You can find the repository in the link below."
            )

            Text(
                "https://github.com/bwqr/mavinote",
                textDecoration = TextDecoration.Underline,
                modifier = Modifier.clickable { localUriHandler.openUri("https://github.com/bwqr/mavinote") }
            )
        }
    }
}
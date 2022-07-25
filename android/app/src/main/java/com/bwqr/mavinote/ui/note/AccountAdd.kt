package com.bwqr.mavinote.ui.note

import android.view.KeyEvent
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.text.KeyboardActions
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material.Button
import androidx.compose.material.OutlinedTextField
import androidx.compose.material.Text
import androidx.compose.runtime.*
import androidx.compose.ui.ExperimentalComposeUiApi
import androidx.compose.ui.Modifier
import androidx.compose.ui.focus.FocusDirection
import androidx.compose.ui.input.key.Key
import androidx.compose.ui.input.key.key
import androidx.compose.ui.input.key.onPreviewKeyEvent
import androidx.compose.ui.platform.LocalFocusManager
import androidx.compose.ui.text.input.ImeAction
import androidx.navigation.NavController
import com.bwqr.mavinote.models.Message
import com.bwqr.mavinote.models.ReaxException
import com.bwqr.mavinote.ui.NoteScreens
import com.bwqr.mavinote.viewmodels.NoteViewModel
import kotlinx.coroutines.launch

@OptIn(ExperimentalComposeUiApi::class)
@Composable
fun AccountAdd(navController: NavController) {
    val scope = rememberCoroutineScope()

    var inProgress by remember { mutableStateOf(false) }

    var email by remember { mutableStateOf("") }
    var password by remember { mutableStateOf("") }

    var warning by remember { mutableStateOf("") }

    val focusManager = LocalFocusManager.current

    Column {
        OutlinedTextField(
            placeholder = { Text("Email") },
            value = email,
            onValueChange = { email = it },
            modifier = Modifier.onPreviewKeyEvent {
                if (it.key == Key.Tab && it.nativeKeyEvent.action == KeyEvent.ACTION_DOWN) {
                    focusManager.moveFocus(FocusDirection.Down)
                    true
                } else {
                    false
                }
            },
            keyboardOptions = KeyboardOptions(imeAction = ImeAction.Next),
            keyboardActions = KeyboardActions(onNext = { focusManager.moveFocus(FocusDirection.Down) })
        )
        OutlinedTextField(
            placeholder = { Text("Password") },
            value = password,
            onValueChange = { password = it },
            modifier = Modifier.onPreviewKeyEvent {
                if (it.key == Key.Tab && it.nativeKeyEvent.action == KeyEvent.ACTION_DOWN) {
                    focusManager.moveFocus(FocusDirection.Down)
                    true
                } else {
                    false
                }
            },
            keyboardOptions = KeyboardOptions(imeAction = ImeAction.Next),
            keyboardActions = KeyboardActions(onNext = { focusManager.moveFocus(FocusDirection.Down) })
        )

        if (warning.isNotEmpty()) {
            Text(text = warning)
        }

        Button(onClick = {
            if (!inProgress) {
                inProgress = true

                scope.launch {
                    try {
                        NoteViewModel().addAccount(email, password)

                        navController.navigate(NoteScreens.Folders.route)
                    } catch (e: ReaxException) {
                        if (e.error is Message) {
                            warning = e.error.message
                        } else {
                            e.handle()
                        }
                    } finally {
                        inProgress = false
                    }
                }
            }
        }) {
            Text("Add account")
        }
    }
}
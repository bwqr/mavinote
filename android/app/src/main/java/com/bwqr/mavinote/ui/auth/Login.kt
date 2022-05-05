package com.bwqr.mavinote.ui.auth

import android.util.Log
import androidx.compose.foundation.layout.Column
import androidx.compose.material.Button
import androidx.compose.material.OutlinedTextField
import androidx.compose.material.Text
import androidx.compose.runtime.*
import androidx.navigation.NavController
import com.bwqr.mavinote.Screen
import com.bwqr.mavinote.models.HttpError
import com.bwqr.mavinote.models.ReaxException
import com.bwqr.mavinote.models.Message
import com.bwqr.mavinote.viewmodels.AuthViewModel
import kotlinx.coroutines.launch

@Composable
fun Login(navController: NavController) {
    val scope = rememberCoroutineScope()

    var inProgress by remember { mutableStateOf(false) }
    var email by remember { mutableStateOf("") }
    var password by remember { mutableStateOf("") }
    var warning by remember { mutableStateOf("") }

    Column {
        OutlinedTextField(placeholder = { Text("Email") }, value = email, onValueChange = { email = it })
        OutlinedTextField(placeholder = { Text("Password") }, value = password, onValueChange = { password = it })

        if (warning.isNotEmpty()) {
            Text(text = warning)
        }

        Button(onClick = {
            if (!inProgress) {
                inProgress = true

                scope.launch {
                    try {
                        AuthViewModel().login(email, password).getOrThrow()

                        navController.navigate(Screen.Folders.route)
                    } catch (e: ReaxException) {
                        if (e.error is Message) {
                            warning = e.error.message
                        } else {
                            Log.e("Login", "unhandled error ${e.error}")
                        }
                    } finally {
                        inProgress = false
                    }
                }
            }
        }) {
            Text("Login")
        }
    }
}
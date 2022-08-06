package com.bwqr.mavinote.viewmodels

import com.bwqr.mavinote.models.*
import kotlinx.coroutines.cancel
import kotlinx.coroutines.channels.awaitClose
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.callbackFlow
import kotlinx.coroutines.suspendCancellableCoroutine
import kotlin.coroutines.resume
import kotlin.coroutines.resumeWithException


class NoteViewModel {
    suspend fun sync(): Unit = suspendCancellableCoroutine { cont ->
        val onceId = Runtime.instance.startOnce(Once(
            onNext = { cont.resume(Unit) },
            onError = { cont.resumeWithException(it) },
            onStart = { _sync(it) }
        ))

        cont.invokeOnCancellation {
            Runtime.instance.abortOnce(onceId)
        }
    }

    fun accounts(): Flow<List<Account>> = callbackFlow {
        val streamId = Runtime.instance.startStream(Stream(
            onNext = { deserializer ->
                trySend(TraitHelpers.deserializeList(deserializer) {
                    Account.deserialize(it)
                })
            },
            onError = { cancel("", it) },
            onStart = { _accounts(it) },
            onComplete = { channel.close() }
        ))

        awaitClose {
            Runtime.instance.abortStream(streamId)
        }
    }

    suspend fun account(accountId: Int): Account? = suspendCancellableCoroutine { cont ->
        val onceId = Runtime.instance.startOnce(Once(
            onNext = { cont.resume(TraitHelpers.deserializeOption(it) { deserializer ->
                Account.deserialize(deserializer)
            }) },
            onError = { cont.resumeWithException(it) },
            onStart = { _account(it, accountId) }
        ))

        cont.invokeOnCancellation {
            Runtime.instance.abortOnce(onceId)
        }
    }

    suspend fun mavinoteAccount(accountId: Int): Mavinote? = suspendCancellableCoroutine { cont ->
        val onceId = Runtime.instance.startOnce(Once(
            onNext = { cont.resume(TraitHelpers.deserializeOption(it) { deserializer ->
                Mavinote.deserialize(deserializer)
            }) },
            onError = { cont.resumeWithException(it) },
            onStart = { _mavinoteAccount(it, accountId) }
        ))

        cont.invokeOnCancellation {
            Runtime.instance.abortOnce(onceId)
        }
    }

    suspend fun addAccount(name: String, email: String, password: String, createAccount: Boolean): Unit = suspendCancellableCoroutine { cont ->
        val onceId = Runtime.instance.startOnce(Once(
            onNext = { cont.resume(Unit) },
            onError = { cont.resumeWithException(it) },
            onStart = { _addAccount(it, name, email, password, createAccount) }
        ))

        cont.invokeOnCancellation {
            Runtime.instance.abortOnce(onceId)
        }
    }

    suspend fun deleteAccount(accountId: Int): Unit = suspendCancellableCoroutine { cont ->
        val onceId = Runtime.instance.startOnce(Once(
            onNext = { cont.resume(Unit)},
            onError = { cont.resumeWithException(it)},
            onStart = { _deleteAccount(it, accountId) }
        ))

        cont.invokeOnCancellation {
            Runtime.instance.abortOnce(onceId)
        }
    }

    suspend fun authorizeMavinoteAccount(accountId: Int, password: String): Unit = suspendCancellableCoroutine { cont ->
        val onceId = Runtime.instance.startOnce(Once(
            onNext = { cont.resume(Unit)},
            onError = { cont.resumeWithException(it)},
            onStart = { _authorizeMavinoteAccount(it, accountId, password) }
        ))

        cont.invokeOnCancellation {
            Runtime.instance.abortOnce(onceId)
        }
    }

    fun folders(): Flow<List<Folder>> = callbackFlow {
        val streamId = Runtime.instance.startStream(Stream(
            onNext = { deserializer ->
                trySend(TraitHelpers.deserializeList(deserializer) {
                    Folder.deserialize(it)
                })
            },
            onError = { cancel("", it) },
            onStart = { _folders(it) },
            onComplete = { channel.close() }
        ))

        awaitClose {
            Runtime.instance.abortStream(streamId)
        }
    }

    suspend fun folder(folderId: Int): Folder? = suspendCancellableCoroutine { cont ->
        val onceId = Runtime.instance.startOnce(Once(
            onNext = { deserializer ->
                cont.resume(TraitHelpers.deserializeOption(deserializer) {
                    Folder.deserialize(it)
                })
            },
            onError = { cont.resumeWithException(it) },
            onStart = { _folder(it, folderId) }
        ))

        cont.invokeOnCancellation {
            Runtime.instance.abortOnce(onceId)
        }
    }

    suspend fun createFolder(accountId: Int, name: String): Unit = suspendCancellableCoroutine { cont ->
        val onceId = Runtime.instance.startOnce(Once(
            onNext = { cont.resume(Unit) },
            onError = { cont.resumeWithException(it) },
            onStart = { _createFolder(it, accountId, name) }
        ))

        cont.invokeOnCancellation {
            Runtime.instance.abortOnce(onceId)
        }
    }

    suspend fun deleteFolder(folderId: Int): Unit = suspendCancellableCoroutine { cont ->
        val onceId = Runtime.instance.startOnce(Once(
            onNext = { cont.resume(Unit)},
            onError = { cont.resumeWithException(it)},
            onStart = { _deleteFolder(it, folderId) }
        ))

        cont.invokeOnCancellation {
            Runtime.instance.abortOnce(onceId)
        }
    }

    fun notes(folderId: Int): Flow<List<Note>> = callbackFlow {
        val streamId = Runtime.instance.startStream(Stream(
            onNext = { deserializer ->
                trySend(TraitHelpers.deserializeList(deserializer) {
                    Note.deserialize(it)
                })
            },
            onError = { cancel("", it) },
            onStart = { _noteSummaries(it, folderId) },
            onComplete = { channel.close() }
        ))

        awaitClose {
            Runtime.instance.abortStream(streamId)
        }
    }

    suspend fun note(noteId: Int): Note? = suspendCancellableCoroutine { cont ->
        val onceId = Runtime.instance.startOnce(Once(
            onNext = { deserializer ->
                cont.resume(TraitHelpers.deserializeOption(deserializer) {
                    Note.deserialize(it)
                })
            },
            onError = { cont.resumeWithException(it) },
            onStart = { _note(it, noteId) }
        ))

        cont.invokeOnCancellation {
            Runtime.instance.abortOnce(onceId)
        }
    }

    suspend fun createNote(folderId: Int, text: String): Int = suspendCancellableCoroutine { cont ->
        val onceId = Runtime.instance.startOnce(Once(
            onNext = { cont.resume(it.deserialize_i32()) },
            onError = { cont.resumeWithException(it) },
            onStart = { _createNote(it, folderId, text) }
        ))

        cont.invokeOnCancellation {
            Runtime.instance.abortOnce(onceId)
        }
    }

    suspend fun updateNote(noteId: Int, text: String): Unit = suspendCancellableCoroutine { cont ->
        val onceId = Runtime.instance.startOnce(Once(
            onNext = { cont.resume(Unit) },
            onError = { cont.resumeWithException(it) },
            onStart = { _updateNote(it, noteId, text) }
        ))

        cont.invokeOnCancellation {
            Runtime.instance.abortOnce(onceId)
        }
    }

    suspend fun deleteNote(noteId: Int): Unit = suspendCancellableCoroutine { cont ->
        val onceId = Runtime.instance.startOnce(Once(
            onNext = { cont.resume(Unit) },
            onError = { cont.resumeWithException(it) },
            onStart = { _deleteNote(it, noteId) }
        ))

        cont.invokeOnCancellation {
            Runtime.instance.abortOnce(onceId)
        }
    }

    private external fun _sync(onceId: Int): Long
    private external fun _accounts(streamId: Int): Long
    private external fun _account(onceId: Int, accountId: Int): Long
    private external fun _mavinoteAccount(onceId: Int, accountId: Int): Long
    private external fun _addAccount(onceId: Int, name: String, email: String, password: String, createAccount: Boolean): Long
    private external fun _deleteAccount(onceId: Int, accountId: Int): Long
    private external fun _authorizeMavinoteAccount(onceId: Int, accountId: Int, password: String): Long
    private external fun _folders(streamId: Int): Long
    private external fun _folder(onceId: Int, folderId: Int): Long
    private external fun _createFolder(onceId: Int, accountId: Int, name: String): Long
    private external fun _deleteFolder(onceId: Int, folderId: Int): Long
    private external fun _noteSummaries(streamId: Int, folderId: Int): Long
    private external fun _note(onceId: Int, noteId: Int): Long
    private external fun _createNote(onceId: Int, folderId: Int, text: String): Long
    private external fun _updateNote(onceId: Int, noteId: Int, text: String): Long
    private external fun _deleteNote(onceId: Int, noteId: Int): Long
}
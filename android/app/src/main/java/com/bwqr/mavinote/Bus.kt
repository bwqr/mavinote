package com.bwqr.mavinote

import kotlin.coroutines.Continuation
import kotlin.coroutines.resume
import kotlin.coroutines.suspendCoroutine

sealed class BusEvent {
    object DisplayNoInternetWarning : BusEvent()
    class UnhandledError(val error: String) : BusEvent()
}

class Bus {
    private var continuation: Continuation<BusEvent>? = null

    companion object {
        private lateinit var singleton: Bus

        private fun instance(): Bus {
            if (!this::singleton.isInitialized) {
                singleton = Bus()
            }

            return singleton
        }

        suspend fun listen(): BusEvent {
            return suspendCoroutine {
                instance().continuation = it
            }
        }

        fun emit(event: BusEvent) {
            instance().continuation?.resume(event)
        }
    }
}
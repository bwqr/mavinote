package com.bwqr.mavinote.viewmodels

import kotlin.coroutines.Continuation
import kotlin.coroutines.resume
import kotlin.coroutines.suspendCoroutine

enum class BusEvent {
    NoInternetConnection
}

interface Listener {
    suspend fun listen(): BusEvent
}

interface Emitter {
    fun emit(event: BusEvent)
}

class Bus: Listener, Emitter {
    private var continuation: Continuation<BusEvent>? = null

    companion object {
        private lateinit var singleton: Bus

        private fun instance(): Bus {
            if (!this::singleton.isInitialized) {
                singleton = Bus()
            }

            return singleton
        }

        fun listener(): Listener {
            return instance()
        }

        fun emitter(): Emitter {
            return instance()
        }
    }

    override suspend fun listen(): BusEvent {
        return suspendCoroutine {
            continuation = it
        }
    }

    override fun emit(event: BusEvent) {
        continuation?.resume(event)
    }
}
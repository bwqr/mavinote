package com.bwqr.mavinote

import android.util.Log
import kotlinx.coroutines.channels.Channel
import kotlinx.coroutines.channels.ReceiveChannel

sealed class BusEvent {
    class ShowMessage(val message: String) : BusEvent()
}

class Bus {
    private var channel: Channel<BusEvent> = Channel(10)

    companion object {
        private lateinit var singleton: Bus

        private fun instance(): Bus {
            if (!this::singleton.isInitialized) {
                singleton = Bus()
            }

            return singleton
        }

        fun listen(): ReceiveChannel<BusEvent> {
            return instance().channel
        }

        fun message(message: String) {
            emit(BusEvent.ShowMessage(message))
        }

        fun emit(event: BusEvent) {
            val res = instance().channel.trySend(event)

            if (!res.isSuccess) {
                Log.w("Bus", "Failed to send message, $res")
            }
        }
    }
}
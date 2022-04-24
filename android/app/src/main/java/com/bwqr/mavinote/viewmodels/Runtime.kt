package com.bwqr.mavinote.viewmodels

import com.bwqr.mavinote.AppConfig

class Runtime private constructor(filesDir: String) {
    companion object {
        lateinit var instance: Runtime

        fun initialize(filesDir: String) {
            if (!this::instance.isInitialized) {
                instance = Runtime(filesDir)
            }
        }
    }

    init {
        System.loadLibrary("reax")

        _init(AppConfig.APP_NAME, AppConfig.API_URL, filesDir)
    }

    private external fun _init(appName: String, apiUrl: String, storageDir: String)
}
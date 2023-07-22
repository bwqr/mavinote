# Mavinote Android

Android application of Mavinote. It provides all of the functionalities of the Mavinote project.

## Prerequisites
Before starting the build, you need to do:

* Complete **android prerequisites** described in [reax](https://github.com/bwqr/mavinote/tree/main/reax) project.
* If you want synchronization, run the [backend](https://github.com/bwqr/mavinote/tree/main/reax) project.
* Make sure that **cargo** and **rustc** are accessible via **PATH** environment variable.
* Install the Android NDK version specified in the [app/build.gradle](https://github.com/bwqr/mavinote/blob/main/android/app/build.gradle) file as **ndkVersion**.

## Configuration
Project has its own configurations defined in the [app/build.gradle](https://github.com/bwqr/mavinote/blob/main/android/app/build.gradle) as **buildConfigField** for both **release** and **debug** variants.
These configurations are:

* **ApiUrl**: This variable contains the URL of backend service.
* **WsUrl**: This is websocket variant of the **ApiUrl**.

This project reads these configurations from **local.properties** for the debug variant. While building the project for the debug variant please make sure that you have entries similar to the ones below.

```
endpoint.apiUrl=<url-of-your-backend-service>
endpoint.wsUrl=<url-of-your-backend-service>
```

When developing with Android Emulator, you can access your loopback with these configs

```
endpoint.apiUrl=http://10.0.2.2:8050/api
endpoint.wsUrl=ws://10.0.2.2:8050/api
```

## Development

After completing the configurations for debug variant, you can open the project in Android Studio and start the development.
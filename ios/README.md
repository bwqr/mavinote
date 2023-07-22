# Mavinote iOS


iOS application of Mavinote. It provides all of the functionalities of the Mavinote project.

## Prerequisites
Before starting the build, you need to do:

* Complete **android prerequisites** described in [reax](https://github.com/bwqr/mavinote/tree/main/reax) project.
* If you want synchronization, run the [backend](https://github.com/bwqr/mavinote/tree/main/reax) project.
* Make sure that **cargo** and **rustc** are accessible via **PATH** environment variable.

## Configuration
Project has its own configurations defined in the `BuildConfs.swift` for both **release** and **debug** variants.
However, this file does not exist in the git history. You can create it by copying `BuildConfs.swift.example` file to `BuildConfs.swift` in the same directory.
The configurations specified in the file are:

* **API_URL**: This variable contains the URL of backend service.
* **WS_URL**: This is websocket variant of the **ApiUrl**.

### Development
After completing the configurations for the debug variant, you can open the project in Xcode and start the development.
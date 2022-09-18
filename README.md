# Mavinote

Simple and open source note-taking application. You can take notes that reside only in your device or use a Mavinote account to synchronize your notes across your devices.

**### Please note that your notes are not stored as encrypted both on your device and on Mavinote servers. Storing notes as encrpyted is in the TODO list.**

Contents of this Readme

1. [Availability](#availability)
2. [Project Goals](#project-goals)
3. [Project Structure](#project-structure)
4. [Running Mavinote Application](#running-mavinote-application)

### Availability

Anyone can build the project by himself/herself and start using the application. The app is also released on [F-Droid](https://f-droid.org/packages/com.bwqr.mavinote/).

[<img src="https://fdroid.gitlab.io/artwork/badge/get-it-on.png" height="80"/>](https://f-droid.org/packages/com.bwqr.mavinote/)

Since releasing applications on [Google Play](https://play.google.com/) and [App Store](https://www.apple.com/app-store/) requires paid developer accounts, I am not planning to release on these stores until users request it.

### Project Goals

The idea that drives the development of Mavinote is to develop a reactive, cross-platform library which contains the common business logic required by frontends.

Gathering common parts of frontend logic into one shared library and using this library in multiple frontends makes the development of native applications easy. It also enables you to use native APIs and adapt to changes in them more easily.

To give an example, consider the gathering geolocation of the user on any kind of platform. You need to use different APIs for each platform you target, like iOS, Android and Windows.
Although the method of gathering geolocation varies, most of the time, the logic you want to apply to the gathered geolocation is same across different platforms. This same logic can be implemented in one shared library and be used across different plaforms.

With these in mind, this project aims to implement a demo application to showcase the idea described in [this](https://github.com/bwqr/reax-rs/) repo. In order to achieve that, the project needs to complete

- [ ] Auto generated function calls for Kotlin, Swift and TypeScript
- [ ] Auto generated serialization and deserialization of models.
- [ ] e2e testing

Besides the main points, The project also aims to develop a fully functional note-taking application. In order to achieve that, the project needs to complete

- [ ] Note taking
    - [ ] Implement a basic markdown editor
    - [ ] Improve conflict resolution
    - [ ] Store notes as encrypted for both local notes and Mavinote account notes
- [x] Finish iOS application
- [ ] Finish desktop application
- [x] Finish web application

### Project Structure

Mavinote contains multiple projects seperated by being a library, backend and frontend implementations. Build instructions can be found in the projects' folders.

* **Reax**

    Reax is a cross-platform **library** meant to be used by frontends. It contains the common business logic required by frontends. It is implemented in [Rust](https://www.rust-lang.org/tr).

* **Backend**

    Backend is a RESTful web service that provides authorization and synchronization of notes across multiple applications. It is implemented in [Rust](https://www.rust-lang.org/tr) and uses [actix-web](https://actix.rs/).

* **Frontends**

    * Android

        Android implementation of Mavinote. Since developing an Android application on Linux is possible, most of the first development in frontend happens in Android.
        Other frontend implementations may receive the updates much later.
        It is implemented in [Kotlin](https://kotlinlang.org/) and uses [Jetpack Compose](https://developer.android.com/jetpack/compose).

    * iOS

        iOS implementation of Mavinote. It is implemented in [Swift](https://developer.apple.com/swift/) and uses [SwiftUI](https://developer.apple.com/xcode/swiftui/).

    * iced

        A desktop app implementation of Mavinote. Desktop app is currently incomplete. It is implemented in [Rust](https://www.rust-lang.org/tr) and uses [iced](https://iced.rs/).


    * Svelte

        A web app implementation of Mavinote. It is implemented in [TypeScript](https://www.typescriptlang.org/) and uses [Svelte](https://svelte.dev/).


### Running Mavinote application

This project contains more than one application like android and ios.
To run one of them, you can refer to frontend applications' README files.
For example, if you want to run android application, you can checkout [android/README.md](https://github.com/bwqr/mavinote/tree/main/android) file.

If you want to synchronize your notes across multiple applications, you need to run the [backend](https://github.com/bwqr/mavinote/tree/main/backend) project.

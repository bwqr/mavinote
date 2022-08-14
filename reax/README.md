# Reax

Shared library for Mavinote frontends. You need to complete prerequisites before building any other frontend project. Besides the general environmental setup, you have different prerequisites for each frontend project.

## Prerequisites

To begin, you must have rust toolchain installed and accessible via **PATH** in your system, specifically **cargo** and **rustc**.
You can look at [rustup](https://rustup.rs/) to set up the rust programming language.


### Android

You need to have two different targets to build **reax** for android.

* **x86_64-linux-android**
* **aarch64-linux-android**

If you have rustup installed on your system, you can install them by typing
```sh
rustup target add <target>
```

Currently, only 64 bit builds are supported. 32 bit builds are not tested.

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


### iOS

Building reax for ios requires to have macOS environment. You also need to have a set of different targets based on your device architecture.
For devices with an arm64 CPU, you should have

* **aarch64-apple-ios** both for running on iOS Simulator and arm64 build

For devices with x86_64 CPU, you should have

* **x86_64-apple-ios** for running on iOS Simulator
* **aarch64-apple-ios** for an arm64 build

If you have rustup installed on your system, you can install them by typing
```sh
rustup target add <target>
```

Then you have to build the project by running
```bash
cargo build --package ios --target <target-you-want-to-build>
```

### Wasm
You need to have **wasm-pack** installed on your system. Please refer to [wasm-pack](https://rustwasm.github.io/wasm-pack) site for installation.
Then you can build the reax for wasm with
```sh
wasm-pack build --target web
```
If you are building reax for **svelte** project. You need to change the `out-dir` of wasm-pack. You can build reax for svelte with
```sh
# Svelte project expects wasm build to be placed in its directory
wasm-pack build --target web --out-dir ../../svelte/wasm wasm
```

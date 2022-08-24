# Mavinote iOS

**### Current Xcode project is configured to be built for x86_64 iOS Simulator. Extra configurations are required to get an arm64 build like Library Search Paths.**

iOS application of Mavinote. This project depends on the **reax** library. Prior to building the project, you need to complete the **ios prerequisites** described in [reax](https://github.com/bwqr/mavinote/tree/main/reax) project.

After completing the reax prerequisites, you need to create some build time configurations. You can create configuration file by copying `BuildConfs.swift.example` file to `BuildConfs.swift`.
``` bash
# Current PWD should be /PATH-TO-REPO/ios
cp Mavinote/BuildConfs.swift.example Mavinote/BuildConfs.swift
```
This configuration file defines build type dependent variables like **backend** project's **BIND_ADDRESS** configuration as **API_URL** variable.

Now you can open the project with Xcode and build the application.

If you want to have your own custom backend service running and want the built application to connect your service, you need to modify the predefined **API_URL** variable defined between `#elseif` and `#endif` blocks in `BuildCons.swift` file.
This block contains the **release** build type configurations.

Note that backend service is only required if you want to use Mavinote account and want to synchronize notes.

### Development

If you want to develop Mavinote application locally, you can modify **debug** build type configurations like **API_URL** defined between `#if DEBUG` and `#elseif` blocks in `BuildConfs.swift` file.
An example of `BuildConfs.swift` file's content

```swift
#if DEBUG
// debug build type configurations
let API_URL="http://127.0.0.1:8050/api"
#elseif
// release build type configurations
// ...
#endif
```
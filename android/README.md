# Mavinote Android

Android application of Mavinote. This project depends on the **reax** library. **Reax** will be automatically build when you build the android project. However you need to complete the **android prerequisites** described in [reax](https://github.com/bwqr/mavinote/tree/main/reax) project.

After completing the prerequisites, you can build the android project from command line or from Android Studio (be sure that **cargo** and **rustc** are available from **PATH** environment variable).

If you want to have your backend service running and want the built application to connect your service, you need to change the predefined endpoint in [app/build.gradle](https://github.com/bwqr/mavinote/blob/main/android/app/build.gradle). These configurations are defined as **buildConfigField**. You can change **ApiUrl** defined in **release** section as you wish.

Note that backend service is only required if you want to use Mavinote account and synchronize notes.

### Development

If you want to develop Mavinote application locally, you need to create or modify **local.properties** file to define your own configuration. Debug builds read the **ApiUrl** variable from this file. The **local.properties** file needs to have content similar to this

```
endpoint.apiUrl=<url-of-your-backend-service>
```

An example would be
```
endpoint.apiUrl=http://10.0.2.2:8050/api
```
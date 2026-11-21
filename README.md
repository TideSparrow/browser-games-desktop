# Browser Game Hub Desktop

An open-source app for browsing the free browser games catalog at [Browser Game Hub](https://browsergamehub.net/?utm_source=github&utm_medium=referral&utm_campaign=browser-games-readme). Open the app to load the live website in a dedicated window. Tauri uses your system webview, so the download stays small. An internet connection is required.

## Download

Choose your platform on [GitHub Releases](https://github.com/TideSparrow/browser-games-desktop/releases/latest).

| Platform | Download | Open |
| --- | --- | --- |
| Windows 10 or 11, x64 | `Browser-Game-Hub-windows-x64-setup.exe` | Run the installer, then open Browser Game Hub from Start |
| macOS 12 or later, Apple silicon | `Browser-Game-Hub-macos-arm64.zip` | Extract the ZIP and open `Browser Game Hub.app` |
| macOS 12 or later, Intel | `Browser-Game-Hub-macos-x64.zip` | Extract the ZIP and open `Browser Game Hub.app` |
| Android 7 or later, ARM64 | `Browser-Game-Hub-android-arm64.apk` | Open the APK on your phone and follow the installation prompt |

The Windows installer downloads WebView2 if needed. The Windows build is unsigned. macOS builds use an ad hoc signature and lack notarization. Your operating system may ask for approval the first time you run the app.

## Build from source

Install Node.js 22 or later and the [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/) for your operating system, including Rust. Run these commands from the repository directory.

```sh
npm ci
npm start
```

On Windows, run `npm run build -- --bundles nsis` to build an installer. On macOS, run `npm run build -- --bundles app` to build the application bundle. Both commands write their output to `src-tauri/target/release/bundle/`.

For Android, install the [Android Tauri prerequisites](https://v2.tauri.app/start/prerequisites/#android), then run `npx tauri android build --apk --target aarch64`. The unsigned APK is written to `src-tauri/gen/android/app/build/outputs/apk/`. Sign release APKs with [Android apksigner](https://developer.android.com/tools/apksigner) before distributing them.

## License

The app source code is available under the [MIT License](LICENSE). The Browser Game Hub name and icon remain the property of their owner. Website content and games are served online and remain subject to their respective terms and licenses.

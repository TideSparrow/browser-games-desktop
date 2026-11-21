<img src="src-tauri/icons/icon.png" width="64" height="64" alt="Browser Game Hub app icon">

# Browser Game Hub

**English** · [简体中文](README.zh-CN.md)

**Free browser games in a lightweight app for Windows, macOS, and Android.**

[Visit the website][website] · [All downloads][releases] · [Report an issue][issues]

[![Latest release](https://img.shields.io/github/v/release/TideSparrow/browser-games-desktop)][releases]
[![MIT license](https://img.shields.io/badge/License-MIT-blue)](LICENSE)
[![Buy Me a Coffee](https://img.shields.io/badge/Buy%20Me%20a%20Coffee-FFDD00?logo=buymeacoffee&logoColor=111111)][coffee]

## About the app

Open Browser Game Hub to browse the live catalog of racing, puzzle, sports, and arcade games in a dedicated window. The app uses Tauri and your system WebView to keep downloads small. The catalog updates with the website and requires an internet connection.

## Download and install

| Platform | Download | Install |
| --- | --- | --- |
| Windows 10/11 (x64) | [Windows installer][windows] | Run the installer and launch from the Start menu. |
| macOS 12+ (Apple silicon) | [Apple silicon ZIP][mac-arm] | Extract and open `Browser Game Hub.app`. |
| macOS 12+ (Intel) | [Intel Mac ZIP][mac-intel] | Extract and open `Browser Game Hub.app`. |
| Android 7+ (ARM64) | [Signed Android APK][android] | Open the APK on your phone and follow the prompts. |

Windows downloads WebView2 during installation if needed. The Windows installer is unsigned. macOS builds are ad hoc signed, and macOS may ask for approval on first launch because they lack notarization.

## Build from source

Install Node.js 22 or later, Rust via rustup, and the [Tauri prerequisites][tauri] for your platform. From the repository root, install dependencies and start the desktop app in development mode.

```sh
npm ci
npm start
```

Run the packaging command for your target platform. Android also requires the [Android SDK and NDK setup][tauri-android].

```sh
# Windows
npm run build -- --bundles nsis

# macOS
npm run build -- --bundles app

# Android (ARM64)
npx tauri android build --apk --target aarch64
```

Desktop bundles are generated under `src-tauri/target/release/bundle/`. Android writes an unsigned APK under `src-tauri/gen/android/app/build/outputs/apk/`. Use [apksigner][apksigner] to sign it before distribution.

## Feedback and support

If you run into a problem, [open a GitHub issue][issues] with your app version, operating system, and the game or page where it happened. If you enjoy the app, you can [buy us a coffee][coffee].

## License

The app source code uses the [MIT License](LICENSE). The Browser Game Hub name and icon remain the property of their owner. Website content and games are subject to their respective terms and licenses.

[website]: https://browsergamehub.net/?utm_source=github&utm_medium=referral&utm_campaign=browser-games-readme
[releases]: https://github.com/TideSparrow/browser-games-desktop/releases/latest
[windows]: https://github.com/TideSparrow/browser-games-desktop/releases/latest/download/Browser-Game-Hub-windows-x64-setup.exe
[mac-arm]: https://github.com/TideSparrow/browser-games-desktop/releases/latest/download/Browser-Game-Hub-macos-arm64.zip
[mac-intel]: https://github.com/TideSparrow/browser-games-desktop/releases/latest/download/Browser-Game-Hub-macos-x64.zip
[android]: https://github.com/TideSparrow/browser-games-desktop/releases/latest/download/Browser-Game-Hub-android-arm64.apk
[issues]: https://github.com/TideSparrow/browser-games-desktop/issues
[coffee]: https://buymeacoffee.com/marklongpcm
[tauri]: https://v2.tauri.app/start/prerequisites/
[tauri-android]: https://v2.tauri.app/start/prerequisites/#android
[apksigner]: https://developer.android.com/tools/apksigner

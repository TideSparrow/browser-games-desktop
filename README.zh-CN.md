<img src="src-tauri/icons/icon.png" width="64" height="64" alt="Browser Game Hub 应用图标">

# Browser Game Hub

[English](README.md) · **简体中文**

**轻量开源浏览器游戏客户端，支持 Windows、macOS 和 Android。**

[打开官网][website] · [全部下载][releases] · [反馈问题][issues]

[![最新版本](https://img.shields.io/github/v/release/TideSparrow/browser-games-desktop)][releases]
[![MIT 许可证](https://img.shields.io/badge/License-MIT-blue)](LICENSE)
[![请我们喝杯咖啡](https://img.shields.io/badge/Buy%20Me%20a%20Coffee-FFDD00?logo=buymeacoffee&logoColor=111111)][coffee]

## 项目介绍

打开应用即可浏览官网上的赛车、益智、体育和街机游戏。客户端基于 Tauri，使用系统 WebView，安装包体积小。游戏库随官网更新，使用时需要联网。

## 下载与安装

| 平台 | 下载 | 安装 |
| --- | --- | --- |
| Windows 10/11（x64） | [Windows 安装包][windows] | 运行安装程序，然后从开始菜单打开。 |
| macOS 12+（Apple 芯片） | [Apple 芯片 Mac 版 ZIP][mac-arm] | 解压后打开 `Browser Game Hub.app`。 |
| macOS 12+（Intel） | [Intel Mac 版 ZIP][mac-intel] | 解压后打开 `Browser Game Hub.app`。 |
| Android 7+（ARM64） | [已签名的 Android APK][android] | 在手机上打开 APK，按系统提示安装。 |

Windows 安装程序会在需要时下载 WebView2。Windows 安装包未签名，macOS 版使用临时签名且未公证，首次启动时系统可能会要求确认。

## 从源码构建

安装 Node.js 22 或以上版本，通过 rustup 安装 Rust，并准备对应平台的 [Tauri 构建环境][tauri]。在仓库根目录安装依赖并启动桌面开发版。

```sh
npm ci
npm start
```

按目标平台运行对应的打包命令。构建 Android 版还需配置 [Android SDK 和 NDK][tauri-android]。

```sh
# Windows
npm run build -- --bundles nsis

# macOS
npm run build -- --bundles app

# Android (ARM64)
npx tauri android build --apk --target aarch64
```

桌面构建产物位于 `src-tauri/target/release/bundle/`。Android 的未签名 APK 位于 `src-tauri/gen/android/app/build/outputs/apk/`，分发前可用 [apksigner][apksigner] 签名。

## 反馈与支持

遇到问题可以 [提交 GitHub Issue][issues]，注明应用版本、操作系统，以及出问题的游戏或页面。如果你喜欢这个项目，欢迎 [请我们喝杯咖啡][coffee]。

## 许可证

客户端源码采用 [MIT 许可证](LICENSE)。Browser Game Hub 的名称和图标归其权利人所有，网站内容和游戏遵循各自的条款及许可。

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

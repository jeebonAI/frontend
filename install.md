# Installing Djibon

Djibon is a modern, secure, and platform-independent Personal Assistance and Communication Tool (PACT) built with Dioxus and Rust. This guide will help you install and run Djibon on different platforms.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (stable version)
- [Dioxus CLI](https://dioxuslabs.com/learn/0.6.3/CLI/installation) (version 0.6.3 or later)

```bash
cargo install dioxus-cli@0.6.3 --locked
```

## Web Version

### Building for Web

To build the web version of Djibon:

```bash
dx build --platform web --features web
```

The built files will be available in `target/dx/djibon-web/release/web/public`.

### Running Locally

To run the web version locally:

```bash
dx serve --platform web --features web
```

This will start a local development server at http://localhost:8080.

### Deploying to Cloudflare Pages

To deploy the web version to Cloudflare Pages:

1. Build the web version:
```bash
dx build --platform web --features web --release
```

2. Deploy to Cloudflare Pages (dev.djibon.com):
```bash
wrangler pages deploy
```

3. Deploy to Cloudflare Pages (demo.djibon.com):
```bash
wrangler pages deploy --project-name=dioxus-demo
```
## Android Version

### Prerequisites for Android

- Android SDK
- Android NDK
- Rust target for Android: `aarch64-linux-android`

```bash
rustup target add aarch64-linux-android
```

### Building for Android

To build the Android version of Djibon:

```bash
dx build --platform android --features mobile
```

The built APK will be available at `target/dx/djibon-web/debug/android/app/app/build/outputs/apk/debug/app-debug.apk`.

### Installing on an Android Device

To install the APK on a connected Android device:

```bash
adb install -r target/dx/djibon-web/debug/android/app/app/build/outputs/apk/debug/app-debug.apk
```

### Building a Release Version

To build a release version for Android:

```bash
dx build --platform android --features mobile --release
```

### Signing the APK

For a proper release, you need to sign the APK:

1. Create a keystore (if you don't have one):
```bash
keytool -genkey -v -keystore djibon.keystore -alias djibon -keyalg RSA -keysize 2048 -validity 10000
```

2. Sign the APK:
```bash
jarsigner -verbose -sigalg SHA1withRSA -digestalg SHA1 -keystore djibon.keystore target/dx/djibon-web/release/android/app/app/build/outputs/apk/release/app-release-unsigned.apk djibon
```

3. Optimize the APK:
```bash
zipalign -v 4 target/dx/djibon-web/release/android/app/app/build/outputs/apk/release/app-release-unsigned.apk djibon.apk
```

## Desktop Version (Coming Soon)

Support for desktop platforms (Windows, macOS, Linux) is planned for future releases.

## Troubleshooting

If you encounter any issues during installation or running Djibon, please check the following:

1. Make sure you have the latest version of Rust and Dioxus CLI installed.
2. For Android builds, ensure that the Android SDK and NDK are properly configured.
3. Check that the required Rust targets are installed.

If you still have issues, please open an issue on the GitHub repository.

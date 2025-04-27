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

There are several ways to install the Djibon app on your Android device:

#### Method 1: Direct Download from download.djibon.com

1. Visit [download.djibon.com](https://download.djibon.com) on your Android device
2. Tap the "Download APK" button
3. When the download completes, tap on the downloaded file in your notifications or file manager
4. If prompted, allow installation from unknown sources
5. Follow the on-screen instructions to complete the installation
6. Once installed, find and tap the Djibon icon in your app drawer to launch the app

#### Method 2: Using ADB (for developers)

If you have your device connected via USB and ADB set up:

```bash
adb install -r target/dx/djibon-web/debug/android/app/app/build/outputs/apk/debug/app-debug.apk
```

#### Method 3: Using a Local Web Server (for testing)

1. Create a simple web server to host the APK:

```bash
# Create a directory for the download page
mkdir -p download-page

# Copy the APK and icon to the download page directory
cp target/dx/djibon-web/debug/android/app/app/build/outputs/apk/debug/app-debug.apk download-page/djibon-app.apk
cp public/djibon-icon.png download-page/

# Use the existing download-page directory which contains the HTML page and APK
# Start a local server to serve the download-page directory
```

2. Access the local server from your Android device (ensure both devices are on the same network):
   - Find your computer's IP address (e.g., using `ip addr show`)
   - On your Android device, open a browser and navigate to `http://YOUR_COMPUTER_IP:PORT`
   - Download and install the APK as described in Method 1

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

### Deploying to download.djibon.com

To make the APK available for download on download.djibon.com:

1. Build the release version of the Android app:
```bash
dx build --platform android --features mobile --release
```

2. Create a download page directory:
```bash
mkdir -p download-page
```

3. Copy the APK and icon to the download page directory:
```bash
cp target/dx/djibon-web/release/android/app/app/build/outputs/apk/debug/app-debug.apk download-page/djibon-app.apk
cp public/djibon-icon.png download-page/
```

4. Create an index.html file in the download-page directory with a download button

5. Deploy to Cloudflare Pages using Wrangler:
```bash
npx wrangler pages deploy download-page --project-name=djibon-download
```

6. Configure custom domain in Cloudflare Pages dashboard:
   - Log in to your Cloudflare account
   - Go to Pages > djibon-download > Custom domains
   - Add custom domain: download.djibon.com
   - Follow the instructions to verify domain ownership and set up DNS records

## Desktop Version (Coming Soon)

Support for desktop platforms (Windows, macOS, Linux) is planned for future releases.

## Troubleshooting

If you encounter any issues during installation or running Djibon, please check the following:

1. Make sure you have the latest version of Rust and Dioxus CLI installed.
2. For Android builds, ensure that the Android SDK and NDK are properly configured.
3. Check that the required Rust targets are installed.

If you still have issues, please open an issue on the GitHub repository.

# Djibon Frontend

A modern, secure, and platform-independent Personal Assistance and Communication Tool (PACT) built with Dioxus and Rust.

## Features

- **Cross-Platform**: Runs on web browsers, Android, and iOS devices (as PWA)
- **Responsive Design**: Built with Bootstrap for a mobile-first experience
- **Secure**: Built with Rust for memory safety and security
- **Fast**: Compiled to WebAssembly for high performance
- **Offline-Capable**: Works even when offline (coming soon)

## Live Demo

- **Web App**: [web.djibon.com](https://web.djibon.com) - Progressive Web App (PWA) version
- **Download Page**: [download.djibon.com](https://download.djibon.com) - Download Android APK or install as PWA

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (stable version)
- [Dioxus CLI](https://dioxuslabs.com/learn/0.6/CLI/installation) (version 0.6.0 or later)

### Installation

See the [Installation Guide](install.md) for detailed instructions on how to install and run Djibon on different platforms.

Quick start for web:

```bash
# Build the web version
dx build --platform web --features web

# Run locally
dx serve --platform web --features web
```

Quick start for Android:

```bash
# Build the Android version
dx build --platform android --features mobile

# Install on a connected Android device
adb install -r target/dx/djibon-web/debug/android/app/app/build/outputs/apk/debug/app-debug.apk
```

Or simply download and install the APK directly on your Android device:

1. Visit [download.djibon.com](https://download.djibon.com) on your Android device
2. Tap the "Download for Android" button
3. Follow the on-screen instructions to install

### Progressive Web App (PWA)

You can also install Djibon as a Progressive Web App on both Android and iOS:

**Android:**
1. Visit [web.djibon.com](https://web.djibon.com) in Chrome
2. Tap the prompt to "Add to Home Screen" or select "Install App" from the menu

**iOS:**
1. Visit [web.djibon.com](https://web.djibon.com) in Safari
2. Tap the Share button (square with arrow)
3. Scroll down and tap "Add to Home Screen"
4. Tap "Add" to confirm

## Development

### Project Structure

- `src/`: Source code
  - `main.rs`: Main application entry point
  - `components/`: UI components
  - `state.rs`: Application state management
- `public/`: Static assets
- `Dioxus.toml`: Dioxus configuration
- `Cargo.toml`: Rust package configuration

### Building

```bash
# For web
dx build --platform web --features web

# For Android
dx build --platform android --features mobile
```

### Testing

```bash
# Run tests
cargo test
```

## Deployment

### Automated Deployment

#### Development Environment

Every push to the `main` branch automatically deploys to the development environment:

- **Dev Preview**: [djibon-dev.pages.dev](https://djibon-dev.pages.dev)

#### Release Deployment

This project uses GitHub Actions for automated releases and deployment. To deploy a new version:

1. Create a git tag with the format `vx.x.x-[keywords]` where:
   - `x.x.x` is the version number (e.g., 0.1.0)
   - `[keywords]` can include any combination of: `web`, `demo`, `ios`, `android`

2. Push the tag to GitHub:
   ```bash
   git tag v0.1.0-web-demo-ios-android
   git push origin v0.1.0-web-demo-ios-android
   ```

3. The GitHub Actions workflow will automatically:
   - Create a GitHub Release with auto-generated release notes
   - Deploy to web.djibon.com if the tag contains `web`
   - Deploy to demo site if the tag contains `demo`
   - Build iOS app if the tag contains `ios`
   - Build Android app if the tag contains `android`
   - Upload build artifacts to the GitHub Release
   - Update the download page with the latest builds

### Manual Deployment

#### Web

Deploy to Cloudflare Pages:

```bash
dx build --release
npx wrangler pages deploy target/dx/djibon/release/web/public --project-name=djibon-web
```

#### Android

See the [Installation Guide](install.md) for detailed instructions on building, signing, and deploying a release APK.

To deploy the Android app to the download site:

```bash
# Build the release version
dx build --platform android --release

# Copy the APK to the download-page folder
cp target/dx/djibon/release/android/app/app/build/outputs/apk/debug/app-debug.apk download-page/djibon-app.apk

# Deploy to download.djibon.com
cd download-page
npx wrangler pages deploy . --project-name=djibon-download
```

Users can then download and install the app directly from [download.djibon.com](https://download.djibon.com).

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
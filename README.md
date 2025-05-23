# Jeebon Frontend

A modern, secure, and platform-independent Personal AI Agent built with Dioxus and Rust.

## Features

- **Cross-Platform**: Runs on web browsers, Android, and iOS devices (as PWA)
- **Responsive Design**: Built with Bootstrap for a mobile-first experience
- **Secure**: Built with Rust for memory safety and security
- **Fast**: Compiled to WebAssembly for high performance
- **Offline-Capable**: Works even when offline (coming soon)

## Live Demo

- **Web App**: [web.jeebon.ai](https://web.jeebon.ai) - Progressive Web App (PWA) version
- **Download Page**: [download.jeebon.ai](https://download.jeebon.ai) - Download Android APK or iOS builds with version selection

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (stable version)
- [Dioxus CLI](https://dioxuslabs.com/learn/0.6/CLI/installation) (version 0.6.0 or later)

### Installation

See the [Installation Guide](install.md) for detailed instructions on how to install and run jeebon on different platforms.

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
adb install -r target/dx/jeebon-web/debug/android/app/app/build/outputs/apk/debug/app-debug.apk
```

Or simply download and install the APK directly on your Android device:

1. Visit [download.jeebon.ai](https://download.jeebon.ai) on your Android device
2. Select your preferred version from the dropdown menu
3. Tap the "Download for Android" button
4. Follow the on-screen instructions to install

### Progressive Web App (PWA)

You can also install jeebon as a Progressive Web App on both Android and iOS:

**Android:**
1. Visit [web.jeebon.ai](https://web.jeebon.ai) in Chrome
2. Tap the prompt to "Add to Home Screen" or select "Install App" from the menu

**iOS:**
1. Visit [web.jeebon.ai](https://web.jeebon.ai) in Safari
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

- **Dev Preview**: [dev.jeebon.ai](https://dev.jeebon.ai)

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
   - Deploy to web.jeebon.ai if the tag contains `web`
   - Deploy to demo site if the tag contains `demo`
   - Build iOS app if the tag contains `ios`
   - Build Android app if the tag contains `android`
   - Upload build artifacts to the GitHub Release
   - Create a PR to update the download page with the latest builds

### Manual Deployment

#### Web

Deploy to Cloudflare Pages:

```bash
dx build --release
npx wrangler pages deploy target/dx/jeebon/release/web/public --project-name=jeebon-web
```

#### Android

See the [Installation Guide](install.md) for detailed instructions on building, signing, and deploying a release APK.

To deploy the Android app to the download site:

```bash
# Build the release version
dx build --platform android --release

# Copy the APK to the download-page folder
cp target/dx/jeebon/release/android/app/app/build/outputs/apk/debug/app-debug.apk download-page/jeebon-app.apk

# Deploy to download.jeebon.ai
npx wrangler pages deploy ./download-page --project-name=jeebon-download
```

Users can then download and install the app directly from [download.jeebon.ai](https://download.jeebon.ai). The download page provides a dropdown menu of all available releases, allowing users to select and download specific versions.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
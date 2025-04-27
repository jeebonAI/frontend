# Djibon Frontend

A modern, secure, and platform-independent Personal Assistance and Communication Tool (PACT) built with Dioxus and Rust.

## Features

- **Cross-Platform**: Runs on web browsers and Android devices
- **Responsive Design**: Built with Bootstrap for a mobile-first experience
- **Secure**: Built with Rust for memory safety and security
- **Fast**: Compiled to WebAssembly for high performance
- **Offline-Capable**: Works even when offline (coming soon)

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
2. Tap the "Download APK" button
3. Follow the on-screen instructions to install

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

### Web

Deploy to Cloudflare Pages:

```bash
dx build --platform web --features web --release
npx wrangler pages deploy target/dx/djibon-web/release/web/public --project-name=dioxus-test
```

### Android

See the [Installation Guide](install.md) for detailed instructions on building, signing, and deploying a release APK.

To deploy the Android app to the download site:

```bash
# Build the release version
dx build --platform android --features mobile --release

# Deploy to download.djibon.com
npx wrangler pages deploy download-page --project-name=djibon-download
```

Users can then download and install the app directly from [download.djibon.com](https://download.djibon.com).

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
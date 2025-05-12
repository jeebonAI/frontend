#!/bin/bash
set -e

# Configuration
NDK_VERSION="26.3.1299413"
ANDROID_SDK_ROOT="$HOME/Android/Sdk"
OPENSSL_DIR="$(pwd)/vendor/android_openssl/prebuilt/arm64-v8a"
MIN_API_LEVEL=24  # Android 7.0 Nougat

# Ensure Android SDK is installed
if [ ! -d "$ANDROID_SDK_ROOT" ]; then
    echo "Android SDK not found. Installing..."
    mkdir -p "$ANDROID_SDK_ROOT"
    curl -o commandlinetools.zip https://dl.google.com/android/repository/commandlinetools-linux-9477386_latest.zip
    unzip commandlinetools.zip -d "$ANDROID_SDK_ROOT"
    mv "$ANDROID_SDK_ROOT/cmdline-tools" "$ANDROID_SDK_ROOT/latest"
    export PATH="$ANDROID_SDK_ROOT/latest/bin:$PATH"
fi

# Install NDK
if [ ! -d "$ANDROID_SDK_ROOT/ndk/$NDK_VERSION" ]; then
    echo "Installing NDK $NDK_VERSION..."
    sdkmanager --install "ndk;$NDK_VERSION"
fi

# Ensure OpenSSL is available
if [ ! -d "$OPENSSL_DIR" ]; then
    echo "Fetching OpenSSL..."
    ./scripts/fetch-openssl.sh
fi

# Set environment variables
export ANDROID_NDK_HOME="$ANDROID_SDK_ROOT/ndk/$NDK_VERSION"
export OPENSSL_DIR="$OPENSSL_DIR"
export OPENSSL_LIB_DIR="$OPENSSL_DIR/lib"
export OPENSSL_INCLUDE_DIR="$OPENSSL_DIR/include"
export PKG_CONFIG_PATH="$OPENSSL_DIR/lib/pkgconfig"
export PKG_CONFIG_SYSROOT_DIR="$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/sysroot"
export PKG_CONFIG_ALLOW_CROSS=1
export CC_aarch64_linux_android="aarch64-linux-android${MIN_API_LEVEL}-clang"
export CXX_aarch64_linux_android="aarch64-linux-android${MIN_API_LEVEL}-clang++"
export AR_aarch64_linux_android="aarch64-linux-android-ar"
export RANLIB_aarch64_linux_android="aarch64-linux-android-ranlib"
export PATH="$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin:$PATH"

# Install cargo-ndk if not present
if ! command -v cargo-ndk &> /dev/null; then
    echo "Installing cargo-ndk..."
    cargo install cargo-ndk
fi

# Build with cargo-ndk
echo "Building for Android..."
cargo ndk --platform $MIN_API_LEVEL --target aarch64-linux-android build --features mobile --profile android-dev

# Verify no JavaScript assets
if ls target/dx/jeebon/debug/android/public/*.js 2>/dev/null; then
    echo "Error: JavaScript assets found in Android build!"
    exit 1
fi
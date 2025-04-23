#!/bin/bash

# Set the Android NDK environment variable
export ANDROID_NDK_HOME=$ANDROID_HOME/ndk/26.3.11579264

# Print environment variables for debugging
echo "ANDROID_HOME: $ANDROID_HOME"
echo "ANDROID_NDK_HOME: $ANDROID_NDK_HOME"

# Check if the NDK directory exists
if [ ! -d "$ANDROID_NDK_HOME" ]; then
    echo "Error: Android NDK directory not found at $ANDROID_NDK_HOME"
    exit 1
fi

# List connected devices
echo "Connected Android devices:"
adb devices

# Run the Dioxus app on the Android emulator
dx run --platform android --device true

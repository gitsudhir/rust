#!/bin/bash

# Set environment variables
export JAVA_HOME=/opt/homebrew/Cellar/openjdk@17/17.0.17
export PATH="/opt/homebrew/opt/openjdk@17/bin:$PATH"
export ANDROID_HOME=~/Library/Android/sdk

# Sign the APK
$ANDROID_HOME/build-tools/36.1.0/apksigner sign \
  --key-pass pass:android \
  --ks-pass pass:android \
  --ks ~/.android/debug.keystore \
  /Users/sudhirkumar/Desktop/sudhir/gitsudhir/rust/video-editor-rs/src-tauri/gen/android/app/build/outputs/apk/universal/release/app-universal-release-unsigned.apk

# Verify the signed APK
$ANDROID_HOME/build-tools/36.1.0/apksigner verify \
  /Users/sudhirkumar/Desktop/sudhir/gitsudhir/rust/video-editor-rs/src-tauri/gen/android/app/build/outputs/apk/universal/release/app-universal-release-unsigned.apk

echo "APK signed and verified successfully!"
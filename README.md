# uba

The Ubiquitous Budgeting App.

## Building for Desktop

use plain cargo:

```sh
cargo run
cargo build --release
```

## Mobile

Rust compilation still goes through cargo
(via [xbuild](https://github.com/rust-mobile/xbuild)). xbuild only handles
platform packaging (APK / IPA) and deployment. App metadata lives in
[`manifest.yaml`](manifest.yaml).

### Setup (once)

```sh
cargo install xbuild

rustup target add aarch64-linux-android
rustup target add aarch64-apple-ios aarch64-apple-ios-sim

x doctor   # lists missing SDK/NDK tools
```

Android: install [Android Studio](https://developer.android.com/studio) and the
SDK/NDK (path shown in SDK settings; set `ANDROID_HOME` if needed).

ios: requires macOS with Xcode.

### Run on a device

```sh
x devices                          # list connected devices / emulators
x run --device <id>                # build (via cargo) and install
x run --device <id> --release
```

Examples of device ids: `adb:emulator-5554` (Android), `sim:…` or `imd:…` (iOS).

### Build installable artifacts

```sh
x build --platform android --arch arm64 --format apk --release
# output: target/x/release/android/.apk

x build --platform ios --release
# output under target/x/release/ios/
```

### Compile-check without a device

```sh
cargo build --target aarch64-linux-android
cargo build --target aarch64-apple-ios
```

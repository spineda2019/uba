# uba

The Ubiquitous Budgeting App.

## Workspace layout

```
uba-core/      platform-agnostic logic (no Slint)
uba-desktop/   desktop + iOS binary
uba-android/   Android cdylib (android_main)
ui/            shared Slint UI (compiled by both app crates)
```

## Building for Desktop

From the workspace root (`default-members` points at `uba-desktop`):

```sh
cargo run
cargo build --release
```

Explicit package:

```sh
cargo run -p uba-desktop
```

## Mobile

Rust compilation still goes through cargo
(via [xbuild](https://github.com/rust-mobile/xbuild)). xbuild builds
`uba-android` (cdylib) for Android and `uba-desktop` (bin) for iOS.
App metadata lives in [`manifest.yaml`](manifest.yaml).

### Setup (once)

```sh
cargo install xbuild

rustup target add aarch64-linux-android
rustup target add aarch64-apple-ios aarch64-apple-ios-sim

x doctor   # lists missing SDK/NDK tools
```

Android: install [Android Studio](https://developer.android.com/studio) and the
SDK/NDK (path shown in SDK settings; set `ANDROID_HOME` if needed).

iOS: requires macOS with Xcode.

### Run on a device

```sh
x devices
x run --device <id>
x run --device <id> --release
```

### Build installable artifacts

```sh
x build --platform android --arch arm64 --format apk --release
# builds uba-android → target/x/release/android/.apk

x build --platform ios --release
# builds uba-desktop bin → target/x/release/ios/
```

### Compile-check without a device

Requires `ANDROID_HOME` and a JDK for Android (Slint compiles Java helpers).

```sh
cargo build -p uba-android --target aarch64-linux-android
cargo build -p uba-desktop --target aarch64-apple-ios
```

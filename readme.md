# rust for android

how to play on ubuntu

- install rust https://rustup.rs

- install some Android targets (arm64, arm, x86_64, x86) for rust.
```
rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
```

- install [Android Studio](https://developer.android.com/studio)

- install Android SDK and NDK

![sdk](https://raw.githubusercontent.com/ssrlive/rust_on_android_ios/main/sdk.png)

- install `cargo-apk` plug-in
```
cargo install cargo-apk
```

- we can play it with our Android Phone now
```
export ANDROID_SDK_ROOT=$HOME/Android/Sdk
export ANDROID_NDK_ROOT=$HOME/Android/Sdk/ndk/23.1.7779620

cargo apk build --example looper
cargo apk run --example looper

```

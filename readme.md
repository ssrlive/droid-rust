# rust for android

how to play on ubuntu

- install rust https://rustup.rs

- install some Android targets (arm64, arm, x86_64, x86) for rust.
```bash
rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
```

- install [Android Studio](https://developer.android.com/studio)

- install Android SDK and NDK

![sdk](https://raw.githubusercontent.com/ssrlive/rust_on_android_ios/main/sdk.png)

- install `cargo-apk` plug-in
```bash
cargo install cargo-apk
```
> To use the latest version, please install it from source code.
> ```bash
> git clone https://github.com/rust-windowing/android-ndk-rs.git ndk-rs
> cargo install --path ./ndk-rs/cargo-apk
> ```

- we can play it with our Android Phone now
```bash
export ANDROID_SDK_ROOT=$HOME/Android/Sdk
export ANDROID_NDK_ROOT=$HOME/Android/Sdk/ndk/24.0.8215888
```
> In windows machine, run
> ```console
> set ANDROID_NDK_ROOT=C:\Users\Administrator\AppData\Local\Android\Sdk\ndk\24.0.8215888
> set ANDROID_SDK_ROOT=C:\Users\Administrator\AppData\Local\Android\Sdk
> ```

```
cargo apk build --example looper
cargo apk run --example looper

```

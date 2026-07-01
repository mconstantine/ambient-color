# Hot-testing

## Installation

Install Android Studio and the NDK. Then, configure the your shell profile file to find it:

```bash
export ANDROID_HOME=$HOME/Android/Sdk
export ANDROID_NDK_HOME=$ANDROID_HOME/ndk/$(ls -1 $ANDROID_HOME/ndk | sort -V | tail -n 1)
```

Install `cargo-ndk` and the Android target for the Android package compilation:

```bash
cargo install cargo-ndk
rustup target add aarch64-linux-android
rustup target add x86_64-linux-android
```

## CLI

Install the `cli` package:

```bash
cargo install --bin cli --path ./cli --root ~/.local
```

Symlink the `theme` and `ambient` scripts:

```bash
ln -s ./theme ~/.local/bin
ln -s ./ambient ~/.local/bin
```

Link the daemon:

```bash
ln -s ./ambient-color.service ~/.config/systemd/user
ln -s ./ambient-color.timer ~/.config/systemd/user
```

Start the service:

```bash
systemctl --user daemon-reload
systemctl --user enable --now ambient-color.timer
```

Manual runs:

- `ambient-color` gets the data, writes the cache, generates the color, compiles the templates, draws the desktop wallpapers and sends the color to Home Assistant
- `ambient-color compile` compiles the templates from the cache
- `ambient-color draw` draws the desktop wallpapers from the cache
- `ambient-color ha` sends the color to Home Assitant from the cache
- `ambient` does the same job as the `systemd` service: it calls `ambient-color` and sets the next timer, which in turn will call `ambient`
- `theme` will set the theme from the cache, then wait for any change to the cache to update it

## WASM

Build the WASM wrapper:

```bash
cd wasm_wrapper

# Builds the WASM wrapper from Rust code
wasm-pack build --target web
```

Run the simulator:

```bash
cd wasm_wrapper/ambient-color
npm start
```

## Android

Build the Android libray with UniFFI:

```bash
cargo build --package android_ffi
cargo run --bin uniffi-bindgen -- generate --library target/debug/libandroid_ffi.so --language kotlin --out-dir android_src
```

Enter the file `./android_src/uniffi/android_ffi/android_ffi.kt` and replace the package name:

```kotlin
package it.mconst.ambientcolor.presentation
```

Compile the NDK bridge:

```bash
# For the Android Wear emulator
cargo ndk -t x86_64 -o ./jniLibs build --release --package android_ffi

# For the actual Android Wear device
cargo ndk -t arm64-v8a -o ./jniLibs build --release --package android_ffi
```

Copy the `jniLibs` directory and `android_ffi.kt` file into your Android Studio project:

```bash
cp -r ./jniLibs ./android_ffi/AmbientColor/wear/src/main
cp -r ./android_src/uniffi/android_ffi/android_ffi.kt ./android_ffi/AmbientColor/wear/src/main/java/it/mconst/ambientcolor/presentation
```

# Hot-testing

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

Test the CLI command:

```bash
cargo build --release --bin cli
./ambient.sh
```

Test the WASM wrapper:

```bash
cd wasm_wrapper

# Builds the WASM wrapper from Rust code
wasm-pack build --target web

# Runs a simple web server on port 8000 to test index.html
python3 -m http.server
```

Build the Android libray with UniFFI:

```bash
cargo build --package android_ffi
cargo run --bin uniffi-bindgen -- generate --library target/debug/libandroid_ffi.so --language kotlin --out-dir android_src
```

Compile the NDK bridge:

```bash
# For the Android Wear emulator
cargo ndk -t x86_64 -o ./jniLibs build --release --package android_ffi

# For the actual Android Wear device
cargo ndk -t arm64-v8a -o ./jniLibs build --release --package android_ffi
```

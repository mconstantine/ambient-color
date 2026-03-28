# Hot-testing

Install `cargo-ndk` and the Android target for the Android package compilation:

```
cargo install cargo-ndk
rustup target add aarch64-linux-android
```

Test the CLI command:

```bash
cargo run build cli
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

```
cargo build --package android_ffi
cargo run --bin uniffi-bindgen -- generate --library target/debug/libandroid_ffi.so --language kotlin --out-dir android_src
```

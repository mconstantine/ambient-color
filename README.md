# Hot-testing

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

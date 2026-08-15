# Hot-testing

## Installation

## CLI

Install the `cli` package:

```bash
cargo install --bin cli --path ./cli --root ~/.local
```

Symlink the `theme` and `ambient` scripts:

> You might want to use absolute paths

```bash
ln -s ./theme ~/.local/bin
ln -s ./ambient ~/.local/bin
```

Link the daemon:

> You might want to use absolute paths

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

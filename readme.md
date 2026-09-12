# ambient-color

Builds a color from the current date, weather and solar times, then themes the whole desktop with it.

The color lives in OKLCH, where each axis is driven by one measurement:

- **hue** — the day of the year, turning once around the wheel per year
- **chroma** — the current temperature, relative to the day's minimum and maximum
- **luma** — the time of day, ramping across the hour around sunrise and sunset

From that color it derives Tailwind-style `50`–`950` palettes (primary, opposite, secondary, tertiary and neutral), compiles them into the config files of the apps in use, draws the desktop wallpapers and sends the color to Home Assistant.

Weather and astronomy data come from [wttr.in](https://wttr.in).

## Related repositories

This repository holds the logic only. Some of what it needs lives elsewhere:

- **the templates** — rendered from `~/.config/ambient-color/templates`, kept in the dotfiles repository
- **`house`** — provides the Home Assistant access token that `ambient-color ha` reads from the keyring (`house auth login`)
- **`dark`, `light` and `night`** — called by the `theme` script to switch the Hyprland theme

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

- `ambient-color` gets the data, generates the color, compiles the templates, writes the cache and sends the color to Home Assistant
- `ambient-color compile` compiles the templates from the cache
- `ambient-color draw` draws the desktop wallpapers from the cache
- `ambient-color ha` sends the color to Home Assistant from the cache
- `ambient` does the same job as the `systemd` service: it calls `ambient-color` and sets the next timer, which in turn will call `ambient`
- `theme` will set the theme from the cache, then wait for any change to the cache to update it

> The wallpapers are not drawn by `ambient-color` on its own: `theme` calls `ambient-color draw` whenever the cache changes

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

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

## Development environment

All the Rust and Node tooling (`cargo`, `wasm-pack`, `npm`) runs inside the `ambient-color` container, so nothing needs to be installed on the host apart from Docker. The repository is mounted at `/repo`, so build output in `target/` and `wasm_wrapper/pkg/` shows up on the host too.

The container joins the external `caddy` network, which has to exist first:

```bash
docker network create caddy # only if it doesn't exist yet
docker compose up -d --build
```

Run commands inside it with `docker container exec`, e.g.:

```bash
docker container exec ambient-color cargo check
docker container exec ambient-color cargo test
```

> Files created by the container are owned by `root`

## Installation

The binary runs on the host, since it needs `hyprctl`, the keyring and the user's directories. The host also needs `jq`, `inotify-tools` and `hyprpaper`.

### CLI

Build the `cli` package in release mode, then copy it to the host as `ambient-color`:

```bash
docker container exec ambient-color ./build cli
cp target/release/cli ~/.local/bin/ambient-color
```

Symlink the `theme` and `ambient` scripts, using absolute paths so the links don't break:

```bash
ln -s "$PWD/theme" ~/.local/bin
ln -s "$PWD/ambient" ~/.local/bin
```

Link the daemon:

```bash
ln -s "$PWD/ambient-color.service" ~/.config/systemd/user
ln -s "$PWD/ambient-color.timer" ~/.config/systemd/user
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
- `ambient` is what the `systemd` service runs: it calls `ambient-color` and schedules the next run of itself with `systemd-run` (every 20 minutes, every 5 around sunrise and sunset)
- `theme` will set the theme from the cache, then wait for any change to the cache to update it

> The wallpapers are not drawn by `ambient-color` on its own: `theme` calls `ambient-color draw` whenever the cache changes

## WASM

Build the WASM wrapper into `wasm_wrapper/pkg`:

```bash
docker container exec ambient-color ./build wasm
```

The simulator depends on `wasm_wrapper/pkg` as a local package, so install its dependencies after the first build:

```bash
docker container exec -w /repo/wasm_wrapper/ambient-color ambient-color npm install
```

Run the simulator:

```bash
docker container exec -w /repo/wasm_wrapper/ambient-color ambient-color npm start
```

Vite listens on port `3000` inside the container. No port is published, so it's reached through Caddy on the `caddy` network.

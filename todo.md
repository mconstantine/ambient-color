# Todo

## Split

- Add a config with "output": a list of "templates", "wallpapers", "home-assistant", so the app will make different things based on the environment
- All apps are deployed from the same binary
- All apps fetch from wttr.in independently, from their own IP, so the location and local time is always right
- Config tasks for `homie`: Home Assistant
- Config tasks for `doomstick`: templates, wallpapers

### Schedule

One unit, one self-rescheduling timer, no `OnCalendar`. Every tick:

1. Fetch from wttr.in and write the cache, but only if the cached data is older than an hour. A failed fetch is not fatal
2. Compute the theme from the cached data, today's date and the current time
3. Write `data.json`
4. Perform the config tasks
5. Schedule the next tick (20 minutes, 5 around sunrise and sunset)

- Two caches: the raw data from wttr.in, and the computed theme in `data.json`. The tick reads the first and writes the second, so the theme never waits for the network
- Drop `ExecStartPre=until ping wttr.in` from the service: boot computes from the cache right away, then refreshes once the network is up
- Stale data only affects the chroma: the hue comes from the date, the luma from the clock, and sunrise and sunset only move 1-2 minutes a day

### Home Assistant

- Use a webhook: `POST /api/webhook/<id>`, with `id` being the "secret". No token
- `local_only` (the default) is enough, since `homie` runs Home Assistant itself
- Drops both `keyring` and the three hosts fallback from `home_assistant.rs`

### Crates

- Split them so `homie` builds without `tiny-skia`, `hyprctl` and `keyring`
- Moving the next tick computation from Bash to `core_logic` is now optional, since all hosts run the same script: still worth it to test it and to drop the `jq` dependency

### Fixes

- `parse.rs:146`: `"35"` looks like a typo for `"305"`
- `WeatherCondition` and `MoonPhase` need a `try_from` to match `into = "String"`: the `Unknown` variants serialize to a bare string that doesn't deserialize, so `read_cache()` can't read back what it wrote

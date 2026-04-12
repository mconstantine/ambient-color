#!/bin/bash

update_theme() {
  if pgrep -x "Hyprland" > /dev/null; then
    data=$(cat ~/.cache/ambient-color/data.json)
    time=$(echo "$data" | jq -r '.time')

    if [[ "$time" == "Sunrise" ]]; then
      dark
    elif [[ "$time" == "Day" ]]; then
      light
    elif [[ "$time" == "Sunset" ]]; then
      dark
    elif [[ "$time" == "Night" ]]; then
      night
    fi

    ambient-color draw
  fi
}

update_theme

inotifywait -m -q -e close_write,moved_to --format "%f" ~/.cache/ambient-color/ | while read -r filename; do
  if [[ "$filename" == "data.json" ]]; then
    update_theme
  fi
done

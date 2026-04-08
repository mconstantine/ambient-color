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
fi

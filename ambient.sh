#!/bin/bash

ambient-color
theme.sh

sunrise=$(echo "$data" | jq -r '.weather_data.sunrise_time')
sunset=$(echo "$data" | jq -r '.weather_data.sunset_time')

sunrise_sec=$(date -d "$sunrise" +%s)
sunset_sec=$(date -d "$sunset" +%s)
now_sec=$(date +%s)

one_hour_before_sunrise=$(( sunrise_sec - 3600 ))
one_hour_after_sunrise=$(( sunrise_sec + 3600 ))
one_hour_before_sunset=$(( sunset_sec - 3600 ))
one_hour_after_sunset=$(( sunset_sec + 3600 ))

delay_sec=1200

check_event() {
  local event_time=$1
  local event_name=$2
  local difference=$(( event_time - now_sec ))

  if [[ $difference -gt 0 ]] && [[ $difference -lt $delay_sec  ]]; then
    delay_sec=$difference
  fi
}

check_event "$one_hour_before_sunrise" "one_hour_before_sunrise"
check_event "$one_hour_after_sunrise" "one_hour_after_sunrise"
check_event "$one_hour_before_sunset" "one_hour_before_sunset"
check_event "$one_hour_after_sunset" "one_hour_after_sunset"

systemd-run --user --on-active="${delay_sec}s" \
  --timer-property=AccuracySec=1s \
  ~/.local/bin/ambient.sh

#!/bin/bash

mkdir -p ./rendered_templates
./target/release/cli

cp ./rendered_templates/tmux.conf ~/.cache/ambient-color/tmux.conf
cp ./rendered_templates/ashell.toml ~/.cache/ambient-color/ashell.toml
cp ./rendered_templates/clipse_theme.json ~/.cache/ambient-color/clipse_theme.json
cp ./rendered_templates/foot.ini ~/.cache/ambient-color/foot.ini
cp ./rendered_templates/hyprland.conf ~/.cache/ambient-color/hyprland.conf
cp ./rendered_templates/fnott.ini ~/.cache/ambient-color/fnott.ini
cp ./rendered_templates/hyprlock.conf ~/.cache/ambient-color/hyprlock.conf
cp ./rendered_templates/gtk.css ~/.cache/ambient-color/gtk.css
cp ./rendered_templates/rofi.rasi ~/.cache/ambient-color/rofi.rasi
cp ./rendered_templates/colors.lua ~/.cache/ambient-color/colors.lua

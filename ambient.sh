#!/bin/zsh

mkdir -p ./rendered_templates
./target/release/cli
cp ./rendered_templates/*(D) ~/.cache/ambient-color

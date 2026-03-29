#!/bin/bash

JSON_OUTPUT=$(./target/release/cli)

RESULT=$(echo "$JSON_OUTPUT" | jq -r '.result')

echo "Rust core output intercepted:"
echo "Number: $RESULT"

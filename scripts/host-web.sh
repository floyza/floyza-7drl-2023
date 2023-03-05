#!/usr/bin/env bash
set -eu
cd wasm || exit
python -m http.server 3000 &
xdg-open "localhost:3000/"
sleep 1
kill %1

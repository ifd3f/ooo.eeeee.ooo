#!/usr/bin/env bash

set -euxo pipefail

mkdir -p frames
ffmpeg -i $1 "frames/out-%03d.jpg"

for f in frames/*; do
    mediatoascii --image-path "$f" --as-text --scale-down 10
done

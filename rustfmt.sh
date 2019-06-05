#!/usr/bin/sh

declare -a source_files=(
	"src/main.rs"
	"src/pong.rs"
	"src/systems/ball_bounce.rs"
	"src/systems/ball_move.rs"
	"src/systems/mod.rs"
	"src/systems/paddle.rs"
	"src/systems/score.rs"
)

for file in "${source_files[@]}"
do
	rustfmt +nightly $file
done

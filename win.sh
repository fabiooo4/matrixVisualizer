#!/bin/sh
# This script is used to cross-compile the game for Windows on Linux.
# See https://bevy-cheatbook.github.io/platforms/windows/wsl2.html#cross-compiling-to-run-windows-native
cargo build --target x86_64-pc-windows-gnu &&
# cp target/x86_64-pc-windows-gnu/debug/matrix_visualizer.exe . &&
exec ./target/x86_64-pc-windows-gnu/debug/matrix_visualizer.exe "$@"

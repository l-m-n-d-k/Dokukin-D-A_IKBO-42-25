#!/usr/bin/env bash
# Запуск эмулятора с обоими параметрами: --vfs и --script.
cd "$(dirname "$0")/.." || exit 1

echo "== --vfs и --script вместе"
cargo run --quiet -- --vfs vfs/minimal.csv --script scripts/startup.emu

#!/usr/bin/env bash
# Ошибки в параметрах командной строки (код возврата выводится после каждого).
cd "$(dirname "$0")/.." || exit 1

echo "== нет значения у --vfs"
cargo run --quiet -- --vfs
echo "код возврата: $?"

echo "== нет значения у --script"
cargo run --quiet -- --script
echo "код возврата: $?"

echo "== неизвестный параметр"
cargo run --quiet -- --foo
echo "код возврата: $?"

echo "== несуществующий скрипт при заданном --vfs"
cargo run --quiet -- --vfs ./vfs --script scripts/missing.emu
echo "код возврата: $?"

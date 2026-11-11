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
cargo run --quiet -- --vfs vfs/minimal.csv --script scripts/missing.emu
echo "код возврата: $?"

echo "== несуществующий файл VFS"
cargo run --quiet -- --vfs vfs/missing.csv
echo "код возврата: $?"

for name in bad_base64 bad_type bad_header; do
    echo "== некорректная VFS: $name"
    cargo run --quiet -- --vfs "vfs/$name.csv"
    echo "код возврата: $?"
done

echo "== vfs-info без загруженной VFS"
echo "vfs-info" | cargo run --quiet

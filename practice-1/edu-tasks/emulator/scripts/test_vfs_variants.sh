#!/usr/bin/env bash
# Работа с разными VFS: минимальный, несколько файлов, 3+ уровня вложенности.
# Хеш из vfs-info сравнивается с sha256sum самого CSV-файла.
cd "$(dirname "$0")/.." || exit 1

for name in minimal files deep; do
    echo "== vfs/$name.csv"
    cargo run --quiet -- --vfs "vfs/$name.csv" --script scripts/vfs_info.emu
    echo "sha256sum: $(sha256sum "vfs/$name.csv" | cut -d' ' -f1)"
done

echo "== полный тест команд (глубокая VFS)"
cargo run --quiet -- --vfs vfs/deep.csv --script scripts/full_demo.emu

#!/usr/bin/env bash
# Проверка каждого параметра по отдельности и порядка аргументов.
cd "$(dirname "$0")/.." || exit 1

echo "== только --vfs (ввод закрыт, эмулятор завершается по Ctrl+D)"
cargo run --quiet -- --vfs ./vfs < /dev/null

echo "== только --script"
cargo run --quiet -- --script scripts/startup.emu

echo "== обратный порядок: --script, затем --vfs"
cargo run --quiet -- --script scripts/startup.emu --vfs ./vfs

echo "== скрипт без exit, затем интерактивный режим"
printf 'cd /tmp\nexit\n' | cargo run --quiet -- --vfs ./vfs --script scripts/no_exit.emu

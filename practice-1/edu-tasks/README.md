# Практическая работа №1 (СДО) — эмулятор командной оболочки, вариант 10

Задание: [docs/docs_pr1.pdf](docs/docs_pr1.pdf), вариант №10.
Код: [emulator](emulator). Язык реализации — Rust.

| Этап | Тема | Статус |
|------|------|--------|
| 1 | REPL | выполнен |
| 2 | Конфигурация | выполнен |
| 3 | VFS | выполнен |

## Общее описание

Эмулятор имитирует работу в командной строке UNIX-подобной ОС. Приглашение к
вводу строится из данных реальной ОС (`username@hostname:dir$`). Ввод делится
на команду и аргументы по пробелам. Виртуальная файловая система загружается
из CSV-файла и целиком хранится в памяти.

## Параметры командной строки

| Параметр | Описание |
|----------|----------|
| `--vfs <путь>` | CSV-файл виртуальной файловой системы |
| `--script <путь>` | стартовый скрипт эмулятора |

Параметры необязательны и могут идти в любом порядке. При запуске эмулятор
выводит все параметры в виде `[debug] имя = значение` (`(not set)`, если
параметр не задан). Неизвестный параметр, параметр без значения, отсутствующий
или некорректный файл VFS или скрипта завершают работу с кодом 1 и сообщением
об ошибке.

## Команды

| Команда | Описание |
|---------|----------|
| `ls [args...]` | заглушка: выводит своё имя и аргументы |
| `cd [args...]` | заглушка: выводит своё имя и аргументы |
| `vfs-info` | имя загруженной VFS и SHA-256 её данных |
| `exit` | завершает эмулятор |

Неизвестная команда выводит `<имя>: command not found`, пустой ввод
игнорируется, `Ctrl+D` завершает работу. Ошибки пишутся в stderr.

## Стартовый скрипт

Текстовый файл, по одной команде эмулятора в строке. Комментарий начинается с
`#` и может стоять как в отдельной строке, так и после команды. Пустые строки
пропускаются. При выполнении на экране отображаются приглашение, введённая
строка и вывод команды. Если скрипт не завершается командой `exit`, эмулятор
переходит в интерактивный режим.

## Формат VFS (CSV)

```csv
path,type,content
/home,dir,
/home/user/docs/report.txt,file,UmVwb3J0Cg==
/image.bin,file,AAEC//6JUE5H
```

- `path` — полный путь от корня, вложенность задаётся разделителем `/`;
- `type` — `dir` (каталог) или `file` (файл);
- `content` — содержимое файла в base64 (для каталога пусто).

Недостающие родительские каталоги создаются автоматически. Поля с запятой
заключаются в двойные кавычки. Файл на диске не изменяется, SHA-256
считается от его исходных байтов. Примеры лежат в `emulator/vfs`:
`minimal.csv` (один файл), `files.csv` (несколько файлов, бинарные данные),
`deep.csv` (6 уровней вложенности), а также некорректные `bad_*.csv` для
проверки ошибок.

## Сборка и тесты

Команды выполняются в папке `emulator`:

```
make build   # cargo build --release
make run     # запуск эмулятора
make test    # cargo test (модульные и интеграционные тесты)
make lint    # cargo fmt --check и cargo clippy
```

Скрипты проверки в реальной ОС (bash), все запускаются из любой папки:

| Скрипт | Что проверяет |
|--------|---------------|
| `scripts/test_all_params.sh` | оба параметра вместе |
| `scripts/test_single_params.sh` | каждый параметр отдельно, порядок аргументов, переход в интерактивный режим |
| `scripts/test_vfs_variants.sh` | минимальная VFS, несколько файлов, глубокая вложенность; сравнение хеша с `sha256sum`; полный тест команд (`scripts/full_demo.emu`) |
| `scripts/test_errors.sh` | ошибки параметров, отсутствующие и некорректные файлы, `vfs-info` без VFS |

## Примеры использования

Интерактивный режим:

```
$ make run
[debug] vfs = (not set)
[debug] script = (not set)
user@host:~/emulator$ ls -l /tmp
ls ["-l", "/tmp"]
user@host:~/emulator$ foo bar
foo: command not found
user@host:~/emulator$ exit 1
exit: too many arguments
user@host:~/emulator$ vfs-info
vfs-info: no VFS loaded
user@host:~/emulator$ exit
```

Запуск со стартовым скриптом и VFS:

```
$ cargo run --quiet -- --vfs vfs/deep.csv --script scripts/full_demo.emu
[debug] vfs = vfs/deep.csv
[debug] script = scripts/full_demo.emu
user@host:~/emulator$ vfs-info
name: deep.csv
sha256: 1686db76b8c8c466b3cca75a2de9f0c6a8e3cf7b99dfa5cc766d3c0aa97aada7
user@host:~/emulator$ vfs-info extra
vfs-info: too many arguments
user@host:~/emulator$ foo bar
foo: command not found
user@host:~/emulator$ exit
```

Ошибка в VFS:

```
$ cargo run --quiet -- --vfs vfs/bad_type.csv
[debug] vfs = vfs/bad_type.csv
[debug] script = (not set)
vfs/bad_type.csv: line 2: /a.txt: unknown type 'link'
```

## Структура проекта

```
emulator/
├── Cargo.toml          манифест (зависимости: sha2, base64)
├── Makefile            build / run / test / lint
├── src/
│   ├── main.rs         запуск, REPL
│   ├── config.rs       параметры командной строки
│   ├── script.rs       выполнение стартового скрипта
│   ├── prompt.rs       приглашение из данных ОС
│   ├── parser.rs       разбор команды и аргументов
│   ├── commands.rs     команды
│   ├── csv.rs          разбор строки CSV
│   └── vfs.rs          VFS в памяти, загрузка из CSV, SHA-256
├── tests/repl.rs       интеграционные тесты
├── scripts/            скрипты реальной ОС и стартовые скрипты эмулятора
└── vfs/                примеры VFS в формате CSV
```

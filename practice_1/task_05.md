# Задача 5. Регистрация пользовательской команды (`reg`)

## Задание

Написать программу для регистрации пользовательской команды — задать правильные права доступа и скопировать программу в `/usr/local/bin`:

```text
./reg banner
```

## Код программы `reg`

```bash
#!/usr/bin/env bash
script="$1"
[[ -z "$script" ]] && { echo "Использование: $0 <имя_программы>" >&2; exit 1; }
[[ ! -f "$script" ]] && { echo "Ошибка: файл $script не найден" >&2; exit 1; }
chmod +x "$script" && sudo install -m 0755 "$script" /usr/local/bin/ && echo "Готово: $script установлен в /usr/local/bin" || { echo "Ошибка: не удалось установить $script" >&2; exit 1; }
```

## Проверка

```text
$ ./reg banner
Готово: banner установлен в /usr/local/bin

$ ls -l /usr/local/bin/banner
-rwxr-xr-x. 1 root root 389 Sep 19 21:56 /usr/local/bin/banner

$ banner "Hello from RTU MIREA!"
+-----------------------+
| Hello from RTU MIREA! |
+-----------------------+
```

После успешной установки команду можно вызывать из любой директории без `./`.

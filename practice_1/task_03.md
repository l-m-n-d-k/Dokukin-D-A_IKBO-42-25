# Задача 3. Программа `banner`

## Задание

Написать программу `banner` средствами bash для вывода текстов в рамке. Размер баннера должен меняться в зависимости от длины текста:

```text
[root@localhost ~]# ./banner "Hello from RTU MIREA!"
+-----------------------+
| Hello from RTU MIREA! |
+-----------------------+
```

Программа проверена в ShellCheck (замечаний нет).

## Код программы `banner`

```bash
#!/usr/bin/env bash
text="$*"
[[ -z "$text" ]] && { echo "Использование: $0 <текст>" >&2; exit 1; }
text=" $text "
border="${text//?/-}"
printf '+%s+\n|%s|\n+%s+\n' "$border" "$text" "$border"
```

## Проверка ShellCheck

```text
$ shellcheck banner
$ echo $?
0
```

Замечаний и предупреждений нет.

## Пример работы

```text
$ ./banner "Hello from RTU MIREA!"
+-----------------------+
| Hello from RTU MIREA! |
+-----------------------+

$ ./banner "Ok"
+----+
| Ok |
+----+
```

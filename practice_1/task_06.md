# Задача 6. Комментарий в первой строке файлов `.c`, `.js`, `.py`

## Задание

Написать программу для проверки наличия комментария в первой строке файлов с расширением `c`, `js` и `py`.

## Код программы

```bash
#!/usr/bin/env bash
shopt -s nullglob
found=0
for f in *.c *.js *.py; do
    ext="${f##*.}"
    case "$ext" in
        c)  re='^[[:space:]]*(//|/\*|\*)' ;;
        js) re='^[[:space:]]*(//|/\*)' ;;
        py) re='^[[:space:]]*#' ;;
    esac
    head -n 1 "$f" | grep -qE "$re" && echo "$f: комментарий есть" || echo "$f: комментария нет"
    found=1
done
(( found )) || echo "Файлы .c/.js/.py не найдены"
```

## Проверка

Тестовые файлы:

```text
a.c:              // hello
                  int x;

hello.c:          #include <stdio.h>
b.c:              int x;

c.py:             #!/usr/bin/env python3
                  print(1)
d.py:             print(1)

e.js:             /* block */
                  var a = 1;
```

Запуск:

```text
$ ./task6
a.c: комментарий есть
b.c: комментария нет
hello.c: комментария нет
c.py: комментарий есть
d.py: комментария нет
e.js: комментарий есть
```

`hello.c` корректно определён как файл без комментария (первая строка — директива `#include`, а не комментарий).

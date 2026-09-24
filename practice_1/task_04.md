# Задача 4. Идентификаторы в файле

## Задание

Написать программу для вывода всех идентификаторов (по правилам C/C++ или Java) в файле без повторений.

## Решение

```bash
grep -oE '\b[A-Za-z_][A-Za-z0-9_]*\b' "$1" | sort -u | tr '\n' ' '
```

## Проверка на `hello.c`

Файл `hello.c`:

```c
#include <stdio.h>

int main(void) {
    printf("Hello, world!\n");
    return 0;
}
```

Запуск:

```text
$ ./task4 hello.c      # или: grep -oE '\b[A-Za-z_][A-Za-z0-9_]*\b' hello.c | sort -u | tr '\n' ' '
h hello include int main n printf return stdio void world
```

Результат соответствует примеру в задании (`h` — от `stdio.h`, `n` — из строкового литерала `"\n"`, `Hello` и `world` — из строки).

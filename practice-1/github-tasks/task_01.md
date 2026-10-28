# Задача 1. Отсортированный список пользователей из `/etc/passwd`

## Задание

Вывести отсортированный в алфавитном порядке список имён пользователей из файла `passwd` (понадобится `grep`).

## Решение

```bash
grep -oE '^[^:]+' /etc/passwd | sort
```

## Пример вывода

```text
abrt
adm
apache
avahi
bin
chrony
clevis
colord
...
```

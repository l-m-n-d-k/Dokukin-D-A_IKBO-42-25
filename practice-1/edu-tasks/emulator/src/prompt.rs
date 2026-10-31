//! Формирование приглашения к вводу из данных реальной ОС.

use std::env;
use std::fs;

/// Возвращает имя пользователя ОС.
fn username() -> String {
    env::var("USER")
        .or_else(|_| env::var("USERNAME"))
        .unwrap_or_else(|_| "user".to_string())
}

/// Возвращает имя хоста ОС.
fn hostname() -> String {
    env::var("HOSTNAME")
        .or_else(|_| env::var("COMPUTERNAME"))
        .or_else(|_| fs::read_to_string("/proc/sys/kernel/hostname"))
        .map(|name| name.trim().to_string())
        .unwrap_or_else(|_| "localhost".to_string())
}

/// Возвращает текущий каталог, заменяя домашний на `~`.
fn current_dir() -> String {
    let cwd = match env::current_dir() {
        Ok(path) => path.display().to_string(),
        Err(_) => return "?".to_string(),
    };
    let home = env::var("HOME").or_else(|_| env::var("USERPROFILE"));
    match home {
        Ok(home) if cwd == home => "~".to_string(),
        Ok(home) => match cwd.strip_prefix(&format!("{home}/")) {
            Some(rest) => format!("~/{rest}"),
            None => cwd,
        },
        Err(_) => cwd,
    }
}

/// Собирает приглашение вида `username@hostname:~$ `.
pub fn build() -> String {
    format!("{}@{}:{}$ ", username(), hostname(), current_dir())
}

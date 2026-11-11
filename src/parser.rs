//! Простой парсер: делит строку ввода на команду и аргументы по пробелам.

/// Разобранная строка ввода.
#[derive(Debug, PartialEq, Eq)]
pub struct Parsed {
    /// Имя команды.
    pub command: String,
    /// Аргументы команды.
    pub args: Vec<String>,
}

/// Разбирает строку. Возвращает `None` для пустой строки.
pub fn parse(line: &str) -> Option<Parsed> {
    let mut words = line.split_whitespace();
    let command = words.next()?.to_string();
    let args = words.map(str::to_string).collect();
    Some(Parsed { command, args })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_line_gives_none() {
        assert_eq!(parse("   \t\n"), None);
    }

    #[test]
    fn splits_command_and_args() {
        let parsed = parse("  ls   -l  /tmp \n").unwrap();
        assert_eq!(parsed.command, "ls");
        assert_eq!(parsed.args, ["-l", "/tmp"]);
    }
}

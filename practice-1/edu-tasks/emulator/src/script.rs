//! Выполнение стартового скрипта с отображением ввода и вывода.

use std::fs;
use std::io;

use crate::commands::{self, Outcome};
use crate::{parser, prompt};

/// Символ начала комментария в скрипте.
const COMMENT: char = '#';

/// Убирает комментарий и пробелы по краям строки скрипта.
fn clean(line: &str) -> &str {
    line.split(COMMENT).next().unwrap_or("").trim()
}

/// Выполняет скрипт построчно, печатая приглашение и введённую строку.
pub fn run(path: &str) -> io::Result<Outcome> {
    let text = fs::read_to_string(path)?;
    for line in text.lines().map(clean).filter(|line| !line.is_empty()) {
        println!("{}{}", prompt::build(), line);
        let Some(parsed) = parser::parse(line) else {
            continue;
        };
        if commands::execute(&parsed) == Outcome::Exit {
            return Ok(Outcome::Exit);
        }
    }
    Ok(Outcome::Continue)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn comment_line_is_empty() {
        assert_eq!(clean("# only comment"), "");
    }

    #[test]
    fn trailing_comment_is_removed() {
        assert_eq!(clean("  ls -l  # list"), "ls -l");
    }
}

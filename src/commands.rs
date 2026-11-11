//! Команды эмулятора. На этапе 1 `ls` и `cd` являются заглушками.

use crate::parser::Parsed;
use crate::vfs::Vfs;

/// Результат выполнения команды.
#[derive(Debug, PartialEq, Eq)]
pub enum Outcome {
    /// Продолжить работу REPL.
    Continue,
    /// Завершить работу эмулятора.
    Exit,
}

/// Выполняет разобранную команду.
pub fn execute(parsed: &Parsed, vfs: Option<&Vfs>) -> Outcome {
    match parsed.command.as_str() {
        "ls" | "cd" => stub(parsed),
        "exit" => exit(parsed),
        "vfs-info" => vfs_info(parsed, vfs),
        other => {
            eprintln!("{other}: command not found");
            Outcome::Continue
        }
    }
}

/// Заглушка: выводит имя команды и её аргументы.
fn stub(parsed: &Parsed) -> Outcome {
    println!("{} {:?}", parsed.command, parsed.args);
    Outcome::Continue
}

/// Команда `exit`: завершает эмулятор.
fn exit(parsed: &Parsed) -> Outcome {
    if parsed.args.is_empty() {
        Outcome::Exit
    } else {
        eprintln!("exit: too many arguments");
        Outcome::Continue
    }
}

/// Служебная команда `vfs-info`: имя и SHA-256 загруженной VFS.
fn vfs_info(parsed: &Parsed, vfs: Option<&Vfs>) -> Outcome {
    if !parsed.args.is_empty() {
        eprintln!("vfs-info: too many arguments");
        return Outcome::Continue;
    }
    match vfs {
        Some(vfs) => {
            println!("name: {}", vfs.name);
            println!("sha256: {}", vfs.sha256);
        }
        None => eprintln!("vfs-info: no VFS loaded"),
    }
    Outcome::Continue
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;

    fn run(line: &str) -> Outcome {
        execute(&parse(line).unwrap(), None)
    }

    #[test]
    fn exit_stops_repl() {
        assert_eq!(run("exit"), Outcome::Exit);
    }

    #[test]
    fn exit_with_args_is_error() {
        assert_eq!(run("exit 1"), Outcome::Continue);
    }

    #[test]
    fn vfs_info_without_vfs_continues() {
        assert_eq!(run("vfs-info"), Outcome::Continue);
    }

    #[test]
    fn unknown_command_continues() {
        assert_eq!(run("foo"), Outcome::Continue);
    }
}

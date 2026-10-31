//! Параметры командной строки эмулятора.

/// Параметры, заданные пользователем при запуске.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Config {
    /// Путь к физическому расположению VFS (`--vfs`).
    pub vfs: Option<String>,
    /// Путь к стартовому скрипту (`--script`).
    pub script: Option<String>,
}

impl Config {
    /// Разбирает аргументы командной строки без имени программы.
    pub fn parse<I>(args: I) -> Result<Config, String>
    where
        I: IntoIterator<Item = String>,
    {
        let mut config = Config::default();
        let mut args = args.into_iter();
        while let Some(arg) = args.next() {
            let slot = match arg.as_str() {
                "--vfs" => &mut config.vfs,
                "--script" => &mut config.script,
                _ => return Err(format!("unknown argument: {arg}")),
            };
            match args.next() {
                Some(value) => *slot = Some(value),
                None => return Err(format!("{arg}: missing value")),
            }
        }
        Ok(config)
    }

    /// Выводит все заданные параметры (отладочный вывод при запуске).
    pub fn print_debug(&self) {
        println!("[debug] vfs = {}", show(&self.vfs));
        println!("[debug] script = {}", show(&self.script));
    }
}

/// Текст значения параметра для отладочного вывода.
fn show(value: &Option<String>) -> &str {
    value.as_deref().unwrap_or("(not set)")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(args: &[&str]) -> Result<Config, String> {
        Config::parse(args.iter().map(|arg| arg.to_string()))
    }

    #[test]
    fn no_args_gives_defaults() {
        assert_eq!(parse(&[]), Ok(Config::default()));
    }

    #[test]
    fn both_params_are_read() {
        let config = parse(&["--vfs", "/data", "--script", "run.emu"]).unwrap();
        assert_eq!(config.vfs.as_deref(), Some("/data"));
        assert_eq!(config.script.as_deref(), Some("run.emu"));
    }

    #[test]
    fn unknown_argument_is_error() {
        assert_eq!(parse(&["-x"]), Err("unknown argument: -x".to_string()));
    }

    #[test]
    fn missing_value_is_error() {
        assert_eq!(parse(&["--vfs"]), Err("--vfs: missing value".to_string()));
    }
}

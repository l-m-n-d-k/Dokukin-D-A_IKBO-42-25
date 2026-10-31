//! Эмулятор командной оболочки UNIX. Этап 2: конфигурация.

mod commands;
mod config;
mod parser;
mod prompt;
mod script;

use std::env;
use std::io::{self, BufRead, Write};
use std::process;

use commands::Outcome;
use config::Config;

/// Код завершения при ошибке в параметрах или скрипте.
const EXIT_FAILURE: i32 = 1;

/// Интерактивный цикл: приглашение, ввод, разбор, выполнение.
fn repl() {
    let stdin = io::stdin();
    let mut input = stdin.lock();
    loop {
        print!("{}", prompt::build());
        // Ошибка сброса буфера не мешает работе диалога.
        let _ = io::stdout().flush();

        let mut line = String::new();
        match input.read_line(&mut line) {
            Ok(0) => {
                println!();
                break;
            }
            Ok(_) => {}
            Err(err) => {
                eprintln!("input error: {err}");
                break;
            }
        }

        let Some(parsed) = parser::parse(&line) else {
            continue;
        };
        if commands::execute(&parsed) == Outcome::Exit {
            break;
        }
    }
}

fn main() {
    let config = Config::parse(env::args().skip(1)).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(EXIT_FAILURE);
    });
    config.print_debug();

    if let Some(path) = &config.script {
        match script::run(path) {
            Ok(Outcome::Exit) => return,
            Ok(Outcome::Continue) => {}
            Err(err) => {
                eprintln!("{path}: {err}");
                process::exit(EXIT_FAILURE);
            }
        }
    }
    repl();
}

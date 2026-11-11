//! Эмулятор командной оболочки UNIX. Этап 3: VFS.

mod commands;
mod config;
mod csv;
mod parser;
mod prompt;
mod script;
mod vfs;

use std::env;
use std::io::{self, BufRead, Write};
use std::process;

use commands::Outcome;
use config::Config;
use vfs::Vfs;

/// Код завершения при ошибке в параметрах, VFS или скрипте.
const EXIT_FAILURE: i32 = 1;

/// Интерактивный цикл: приглашение, ввод, разбор, выполнение.
fn repl(vfs: Option<&Vfs>) {
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
        if commands::execute(&parsed, vfs) == Outcome::Exit {
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

    let vfs = config.vfs.as_deref().map(Vfs::load).transpose();
    let vfs = vfs.unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(EXIT_FAILURE);
    });

    if let Some(path) = &config.script {
        match script::run(path, vfs.as_ref()) {
            Ok(Outcome::Exit) => return,
            Ok(Outcome::Continue) => {}
            Err(err) => {
                eprintln!("{path}: {err}");
                process::exit(EXIT_FAILURE);
            }
        }
    }
    repl(vfs.as_ref());
}

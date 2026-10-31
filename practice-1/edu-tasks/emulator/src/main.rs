//! Эмулятор командной оболочки UNIX. Этап 1: REPL.

mod commands;
mod parser;
mod prompt;

use std::io::{self, BufRead, Write};

use commands::Outcome;

fn main() {
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

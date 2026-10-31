//! Интеграционные тесты REPL: запуск бинарника с вводом через stdin.

use std::io::Write;
use std::process::{Command, Stdio};

fn run_session(input: &str) -> (String, String) {
    let mut child = Command::new(env!("CARGO_BIN_EXE_emulator"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to start emulator");
    child
        .stdin
        .take()
        .unwrap()
        .write_all(input.as_bytes())
        .unwrap();
    let output = child.wait_with_output().unwrap();
    (
        String::from_utf8_lossy(&output.stdout).into_owned(),
        String::from_utf8_lossy(&output.stderr).into_owned(),
    )
}

#[test]
fn stubs_print_name_and_args() {
    let (out, _) = run_session("ls -l /tmp\ncd /home\nexit\n");
    assert!(out.contains("ls [\"-l\", \"/tmp\"]"));
    assert!(out.contains("cd [\"/home\"]"));
}

#[test]
fn unknown_command_reports_error() {
    let (_, err) = run_session("foo bar\nexit\n");
    assert!(err.contains("foo: command not found"));
}

#[test]
fn exit_stops_processing() {
    let (out, _) = run_session("exit\nls\n");
    assert!(!out.contains("ls ["));
}

#[test]
fn eof_ends_session() {
    let (out, _) = run_session("ls\n");
    assert!(out.contains("ls []"));
}

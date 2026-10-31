//! Интеграционные тесты REPL: запуск бинарника с вводом через stdin.

use std::io::Write;
use std::process::{Command, Stdio};

fn run_session(input: &str) -> (String, String) {
    run_with_args(&[], input)
}

fn run_with_args(args: &[&str], input: &str) -> (String, String) {
    let mut child = Command::new(env!("CARGO_BIN_EXE_emulator"))
        .args(args)
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

#[test]
fn debug_output_shows_params() {
    let (out, _) = run_with_args(&["--vfs", "vfs/minimal.csv"], "exit\n");
    assert!(out.contains("[debug] vfs = vfs/minimal.csv"));
    assert!(out.contains("[debug] script = (not set)"));
}

#[test]
fn script_shows_input_and_output() {
    let script = "scripts/startup.emu";
    let (out, err) = run_with_args(&["--script", script], "");
    assert!(out.contains("$ ls -l /tmp\nls [\"-l\", \"/tmp\"]"));
    assert!(err.contains("foo: command not found"));
}

#[test]
fn script_comments_are_not_executed() {
    let (out, _) = run_with_args(&["--script", "scripts/startup.emu"], "");
    assert!(!out.contains("Стартовый"));
}

#[test]
fn missing_script_is_error() {
    let (_, err) = run_with_args(&["--script", "nope.emu"], "");
    assert!(err.contains("nope.emu"));
}

#[test]
fn unknown_param_is_error() {
    let (_, err) = run_with_args(&["--foo"], "");
    assert!(err.contains("unknown argument: --foo"));
}

#[test]
fn vfs_info_shows_name_and_hash() {
    let (out, _) = run_with_args(&["--vfs", "vfs/minimal.csv"], "vfs-info\nexit\n");
    assert!(out.contains("name: minimal.csv"));
    let hash = out.split("sha256: ").nth(1).unwrap();
    assert!(hash[..64].chars().all(|ch| ch.is_ascii_hexdigit()));
}

#[test]
fn vfs_info_works_for_nested_vfs() {
    let (out, _) = run_with_args(&["--vfs", "vfs/deep.csv"], "vfs-info\nexit\n");
    assert!(out.contains("name: deep.csv"));
}

#[test]
fn vfs_info_without_vfs_is_error() {
    let (_, err) = run_session("vfs-info\nexit\n");
    assert!(err.contains("vfs-info: no VFS loaded"));
}

#[test]
fn broken_vfs_is_error() {
    let (_, err) = run_with_args(&["--vfs", "vfs/bad_base64.csv"], "");
    assert!(err.contains("bad base64"));
}

#[test]
fn missing_vfs_file_is_error() {
    let (_, err) = run_with_args(&["--vfs", "vfs/missing.csv"], "");
    assert!(err.contains("vfs/missing.csv"));
}

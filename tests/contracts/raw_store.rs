#![allow(dead_code)]

#[path = "../../src/core/command.rs"]
mod command;
#[path = "../../src/core/raw_store.rs"]
mod raw_store;
#[path = "../../src/core/shell.rs"]
mod shell;

use command::CommandSpec;
use raw_store::{RawOutput, RawRenderMode, RawStore};
use shell::ShellCommandExt;
use std::fs;
use std::path::PathBuf;

fn command<const N: usize>(program: &str, args: [&str; N]) -> CommandSpec {
    CommandSpec::new(program, args.iter().map(|arg| (*arg).to_string()).collect())
}

fn temp_store_dir(test_name: &str) -> PathBuf {
    let unique = format!(
        "tss-{test_name}-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    );
    std::env::temp_dir().join(unique)
}

#[test]
fn raw_store_persists_and_retrieves_full_command_output_metadata() {
    let dir = temp_store_dir("roundtrip");
    let store = RawStore::new(&dir);
    let command = command("cargo", ["test", "--all-targets"]);
    let raw = RawOutput::from_parts(
        b"running tests\n".to_vec(),
        b"warning: slow test\n".to_vec(),
        b"running tests\nwarning: slow test\n".to_vec(),
        101,
    );

    let stored = store.store(&command, &raw).expect("store raw output");
    let retrieved = store.get(&stored.id).expect("retrieve raw output");

    assert_eq!(retrieved.id, stored.id);
    assert_eq!(retrieved.stdout, b"running tests\n");
    assert_eq!(retrieved.stderr, b"warning: slow test\n");
    assert_eq!(retrieved.combined, b"running tests\nwarning: slow test\n");
    assert_eq!(retrieved.exit_code, 101);
    assert_eq!(retrieved.cwd, std::env::current_dir().unwrap());
    assert_eq!(retrieved.command_hash, command.command_hash());
    assert!(retrieved.timestamp_unix_millis > 0);

    let _ = fs::remove_dir_all(dir);
}

#[test]
fn raw_ids_are_independent_of_command_text() {
    let dir = temp_store_dir("id-redaction");
    let store = RawStore::new(&dir);
    let command = command("deploy-secret-token", ["--password", "super-secret-value"]);
    let raw = RawOutput::new(b"ok\n".to_vec(), Vec::new(), 0);

    let stored = store.store(&command, &raw).expect("store raw output");

    assert!(!stored.id.contains("deploy"));
    assert!(!stored.id.contains("secret"));
    assert!(!stored.id.contains("password"));
    assert!(!stored.id.contains("super"));

    let _ = fs::remove_dir_all(dir);
}

#[cfg(unix)]
#[test]
fn raw_store_creates_private_directory_and_file() {
    use std::os::unix::fs::PermissionsExt;

    let dir = temp_store_dir("private-mode");
    let store = RawStore::new(&dir);
    let raw = RawOutput::new(b"secret terminal output\n".to_vec(), Vec::new(), 0);

    let stored = store
        .store(&command("cat", ["private.log"]), &raw)
        .expect("store private raw output");
    let file = dir.join(format!("{}.raw", stored.id));

    assert_eq!(
        fs::metadata(&dir).unwrap().permissions().mode() & 0o777,
        0o700
    );
    assert_eq!(
        fs::metadata(&file).unwrap().permissions().mode() & 0o777,
        0o600
    );

    let _ = fs::remove_dir_all(dir);
}

#[test]
fn raw_store_renders_recovery_modes_without_re_filtering() {
    let dir = temp_store_dir("render-modes");
    let store = RawStore::new(&dir);
    let command = command("cargo", ["check"]);
    let raw = RawOutput::from_parts(
        b"stdout line\n".to_vec(),
        b"stderr line\n".to_vec(),
        b"stderr line\nstdout line\n".to_vec(),
        1,
    );

    let stored = store.store(&command, &raw).expect("store raw output");

    assert_eq!(
        store.render(&stored.id, RawRenderMode::Stdout).unwrap(),
        b"stdout line\n"
    );
    assert_eq!(
        store.render(&stored.id, RawRenderMode::Stderr).unwrap(),
        b"stderr line\n"
    );
    assert_eq!(
        store.render(&stored.id, RawRenderMode::Combined).unwrap(),
        b"stderr line\nstdout line\n"
    );

    let full = String::from_utf8(store.render(&stored.id, RawRenderMode::Full).unwrap()).unwrap();
    assert!(full.contains("exit_code: 1"));
    assert!(full.contains("stdout line"));
    assert!(full.contains("stderr line"));

    let _ = fs::remove_dir_all(dir);
}

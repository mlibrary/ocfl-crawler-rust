use anyhow::Result;
use assert_cmd::Command;
use pretty_assertions::assert_eq;
use std::{borrow::Cow, fs};

const PRG: &str = "ocfl-crawler-rust";


// --------------------------------------------------
fn format_file_name(expected_file: &str) -> Cow<str> {
    // Equivalent to: Cow::Borrowed(expected_file)
    expected_file.into()
}

// --------------------------------------------------
fn run(args: &[&str], expected_file_out: &str, expected_file_err: &str) -> Result<()> {
    let file_out = format_file_name(expected_file_out);
    let contents_out = fs::read_to_string(file_out.as_ref())?;
    let expected_out: Vec<&str> =
        contents_out.split('\n').filter(|s| !s.is_empty()).collect();

    let file_err = format_file_name(expected_file_err);
    let contents_err = fs::read_to_string(file_err.as_ref())?;
    let expected_err: Vec<&str> =
        contents_err.split('\n').filter(|s| !s.is_empty()).collect();

    let cmd = Command::cargo_bin(PRG)?.args(args).assert().success();
    let out = cmd.get_output();
    let stdout = String::from_utf8(out.stdout.clone())?;
    let stderr = String::from_utf8(out.stderr.clone())?;
    let lines_out: Vec<&str> =
        stdout.split('\n').filter(|s| !s.is_empty()).collect();
    let lines_err: Vec<&str> =
        stderr.split('\n').filter(|s| !s.is_empty()).collect();

    assert_eq!(lines_out, expected_out);
    assert_eq!(lines_err, expected_err);

    Ok(())
}


// --------------------------------------------------
#[test]
fn storage_roots() -> Result<()> {
    run(&["list", "tests/cli/1.0", "tests/cli/1.1", "tests/cli/1.2"], "tests/cli/expected/storage_roots.out", "tests/cli/expected/storage_roots.err")
}

#[test]
fn storage_roots_absolute() -> Result<()> {
    run(
        &["list", "--absolute", "tests/cli/1.0", "tests/cli/1.1", "tests/cli/1.2"],
        "tests/cli/expected/storage_roots_absolute.out",
        "tests/cli/expected/storage_roots.err",
    )
}

#[test]
fn storage_roots_all_options() -> Result<()> {
    run(
        &[
            "list",
            "--absolute",
            "--identifier",
            "--key",
            "--namespace",
            "ns1",
            "tests/cli/1.0",
            "tests/cli/1.1",
            "tests/cli/1.2",
        ],
        "tests/cli/expected/storage_roots_all.out",
        "tests/cli/expected/storage_roots.err",
    )
}

#[test]
fn storage_roots_identifier() -> Result<()> {
    run(
        &["list", "--identifier", "tests/cli/1.0", "tests/cli/1.1", "tests/cli/1.2"],
        "tests/cli/expected/storage_roots_identifier.out",
        "tests/cli/expected/storage_roots.err",
    )
}

#[test]
fn storage_roots_key() -> Result<()> {
    run(
        &["list", "--key", "tests/cli/1.0", "tests/cli/1.1", "tests/cli/1.2"],
        "tests/cli/expected/storage_roots_key.out",
        "tests/cli/expected/storage_roots.err",
    )
}

#[test]
fn storage_roots_namespace() -> Result<()> {
    run(
        &["list", "--namespace", "ns1", "tests/cli/1.0", "tests/cli/1.1", "tests/cli/1.2"],
        "tests/cli/expected/storage_roots_namespace.out",
        "tests/cli/expected/storage_roots.err",
    )
}


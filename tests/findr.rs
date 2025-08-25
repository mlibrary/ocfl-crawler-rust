use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;
use pretty_assertions::assert_eq;
use rand::{distr::Alphanumeric, Rng};
use std::{borrow::Cow, fs, path::Path};

const PRG: &str = "findr";

// --------------------------------------------------
fn gen_bad_file() -> String {
    loop {
        let filename: String = rand::rng()
            .sample_iter(Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();

        if fs::metadata(&filename).is_err() {
            return filename;
        }
    }
}

// --------------------------------------------------
#[test]
fn skips_bad_dir() -> Result<()> {
    let bad = gen_bad_file();
    let expected = format!("{}: .* [(]os error [23][)]", &bad);
    Command::cargo_bin(PRG)?
        .arg(&bad)
        .assert()
        .success()
        .stderr(predicate::str::is_match(expected)?);
    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_bad_name() -> Result<()> {
    Command::cargo_bin(PRG)?
        .args(["--name", "*.csv"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("error: invalid value '*.csv'"));
    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_bad_type() -> Result<()> {
    let expected = "error: invalid value 'x' for '--type [<TYPE>...]'";
    Command::cargo_bin(PRG)?
        .args(["--type", "x"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(expected));
    Ok(())
}

// --------------------------------------------------
#[cfg(windows)]
fn format_file_name(expected_file: &str) -> Cow<str> {
    // Equivalent to: Cow::Owned(format!("{}.windows", expected_file))
    format!("{}.windows", expected_file).into()
}

// --------------------------------------------------
#[cfg(not(windows))]
fn format_file_name(expected_file: &str) -> Cow<str> {
    // Equivalent to: Cow::Borrowed(expected_file)
    expected_file.into()
}

// --------------------------------------------------
fn run(args: &[&str], expected_file: &str) -> Result<()> {
    let file = format_file_name(expected_file);
    let contents = fs::read_to_string(file.as_ref())?;
    let mut expected: Vec<&str> =
        contents.split('\n').filter(|s| !s.is_empty()).collect();
    expected.sort();

    let cmd = Command::cargo_bin(PRG)?.args(args).assert().success();
    let out = cmd.get_output();
    let stdout = String::from_utf8(out.stdout.clone())?;
    let mut lines: Vec<&str> =
        stdout.split('\n').filter(|s| !s.is_empty()).collect();
    lines.sort();

    assert_eq!(lines, expected);

    Ok(())
}

// --------------------------------------------------
#[test]
fn path1() -> Result<()> {
    run(&["tests/findr/inputs"], "tests/findr/expected/path1.txt")
}

// --------------------------------------------------
#[test]
fn path_a() -> Result<()> {
    run(&["tests/findr/inputs/a"], "tests/findr/expected/path_a.txt")
}

// --------------------------------------------------
#[test]
fn path_a_b() -> Result<()> {
    run(&["tests/findr/inputs/a/b"], "tests/findr/expected/path_a_b.txt")
}

// --------------------------------------------------
#[test]
fn path_d() -> Result<()> {
    run(&["tests/findr/inputs/d"], "tests/findr/expected/path_d.txt")
}

// --------------------------------------------------
#[test]
fn path_a_b_d() -> Result<()> {
    run(
        &["tests/findr/inputs/a/b", "tests/findr/inputs/d"],
        "tests/findr/expected/path_a_b_d.txt",
    )
}

// --------------------------------------------------
#[test]
fn type_f() -> Result<()> {
    run(&["tests/findr/inputs", "-t", "f"], "tests/findr/expected/type_f.txt")
}

// --------------------------------------------------
#[test]
fn type_f_path_a() -> Result<()> {
    run(
        &["tests/findr/inputs/a", "-t", "f"],
        "tests/findr/expected/type_f_path_a.txt",
    )
}

// --------------------------------------------------
#[test]
fn type_f_path_a_b() -> Result<()> {
    run(
        &["tests/findr/inputs/a/b", "--type", "f"],
        "tests/findr/expected/type_f_path_a_b.txt",
    )
}

// --------------------------------------------------
#[test]
fn type_f_path_d() -> Result<()> {
    run(
        &["tests/findr/inputs/d", "--type", "f"],
        "tests/findr/expected/type_f_path_d.txt",
    )
}

// --------------------------------------------------
#[test]
fn type_f_path_a_b_d() -> Result<()> {
    run(
        &["tests/findr/inputs/a/b", "tests/findr/inputs/d", "--type", "f"],
        "tests/findr/expected/type_f_path_a_b_d.txt",
    )
}

// --------------------------------------------------
#[test]
fn type_d() -> Result<()> {
    run(&["tests/findr/inputs", "-t", "d"], "tests/findr/expected/type_d.txt")
}

// --------------------------------------------------
#[test]
fn type_d_path_a() -> Result<()> {
    run(
        &["tests/findr/inputs/a", "-t", "d"],
        "tests/findr/expected/type_d_path_a.txt",
    )
}

// --------------------------------------------------
#[test]
fn type_d_path_a_b() -> Result<()> {
    run(
        &["tests/findr/inputs/a/b", "--type", "d"],
        "tests/findr/expected/type_d_path_a_b.txt",
    )
}

// --------------------------------------------------
#[test]
fn type_d_path_d() -> Result<()> {
    run(
        &["tests/findr/inputs/d", "--type", "d"],
        "tests/findr/expected/type_d_path_d.txt",
    )
}

// --------------------------------------------------
#[test]
fn type_d_path_a_b_d() -> Result<()> {
    run(
        &["tests/findr/inputs/a/b", "tests/findr/inputs/d", "--type", "d"],
        "tests/findr/expected/type_d_path_a_b_d.txt",
    )
}

// --------------------------------------------------
#[test]
fn type_l() -> Result<()> {
    run(&["tests/findr/inputs", "-t", "l"], "tests/findr/expected/type_l.txt")
}

// --------------------------------------------------
#[test]
fn type_f_l() -> Result<()> {
    run(
        &["tests/findr/inputs", "-t", "l", "f"],
        "tests/findr/expected/type_f_l.txt",
    )
}

// --------------------------------------------------
#[test]
fn name_csv() -> Result<()> {
    run(
        &["tests/findr/inputs", "-n", ".*[.]csv"],
        "tests/findr/expected/name_csv.txt",
    )
}

// --------------------------------------------------
#[test]
fn name_csv_mp3() -> Result<()> {
    run(
        &["tests/findr/inputs", "-n", ".*[.]csv", "-n", ".*[.]mp3"],
        "tests/findr/expected/name_csv_mp3.txt",
    )
}

// --------------------------------------------------
#[test]
fn name_txt_path_a_d() -> Result<()> {
    run(
        &["tests/findr/inputs/a", "tests/findr/inputs/d", "--name", ".*.txt"],
        "tests/findr/expected/name_txt_path_a_d.txt",
    )
}

// --------------------------------------------------
#[test]
fn name_a() -> Result<()> {
    run(&["tests/findr/inputs", "-n", "a"], "tests/findr/expected/name_a.txt")
}

// --------------------------------------------------
#[test]
fn type_f_name_a() -> Result<()> {
    run(
        &["tests/findr/inputs", "-t", "f", "-n", "a"],
        "tests/findr/expected/type_f_name_a.txt",
    )
}

// --------------------------------------------------
#[test]
fn type_d_name_a() -> Result<()> {
    run(
        &["tests/findr/inputs", "--type", "d", "--name", "a"],
        "tests/findr/expected/type_d_name_a.txt",
    )
}

// --------------------------------------------------
#[test]
fn path_g() -> Result<()> {
    run(&["tests/findr/inputs/g.csv"], "tests/findr/expected/path_g.txt")
}

// --------------------------------------------------
#[test]
#[cfg(not(windows))]
fn unreadable_dir() -> Result<()> {
    let dirname = "tests/findr/inputs/cant-touch-this";
    if !Path::new(dirname).exists() {
        fs::create_dir(dirname)?;
    }

    std::process::Command::new("chmod")
        .args(["000", dirname])
        .status()
        .expect("failed");

    let cmd = Command::cargo_bin(PRG)?
        .arg("tests/findr/inputs")
        .assert()
        .success();
    fs::remove_dir(dirname)?;

    let out = cmd.get_output();
    let stdout = String::from_utf8(out.stdout.clone())?;
    let lines: Vec<&str> =
        stdout.split('\n').filter(|s| !s.is_empty()).collect();

    assert_eq!(lines.len(), 17);

    let stderr = String::from_utf8(out.stderr.clone())?;
    assert!(stderr.contains("cant-touch-this: Permission denied"));
    Ok(())
}

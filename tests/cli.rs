use anyhow::Result;
// use predicates::prelude::*;
use pretty_assertions::assert_eq;
use std::{borrow::Cow, fs};

const PRG: &str = "ocfl-crawler-rust";

// // --------------------------------------------------
// fn gen_bad_file() -> String {
//     loop {
//         let filename: String = rand::rng()
//             .sample_iter(Alphanumeric)
//             .take(7)
//             .map(char::from)
//             .collect();
//
//         if fs::metadata(&filename).is_err() {
//             return filename;
//         }
//     }
// }
//
// // --------------------------------------------------
// #[test]
// fn skips_bad_dir() -> Result<()> {
//     let bad = gen_bad_file();
//     let expected = format!("{}.*is not a storage root", &bad);
//     Command::cargo_bin(PRG)?
//         .arg(&bad)
//         .assert()
//         .success()
//         .stderr(predicate::str::is_match(expected)?);
//     Ok(())
// }
//
// // --------------------------------------------------
// #[test]
// fn dies_bad_name() -> Result<()> {
//     Command::cargo_bin(PRG)?
//         .args(["--name", "*.csv"])
//         .assert()
//         .failure()
//         .stderr(predicate::str::contains("error: invalid value '*.csv'"));
//     Ok(())
// }
//
// // --------------------------------------------------
// #[test]
// fn dies_bad_type() -> Result<()> {
//     let expected = "error: invalid value 'x' for '--type [<TYPE>...]'";
//     Command::cargo_bin(PRG)?
//         .args(["--type", "x"])
//         .assert()
//         .failure()
//         .stderr(predicate::str::contains(expected));
//     Ok(())
// }

// --------------------------------------------------
fn format_file_name(expected_file: &str) -> Cow<str> {
    // Equivalent to: Cow::Borrowed(expected_file)
    expected_file.into()
}

// --------------------------------------------------
fn run(args: &[&str], expected_file_out: &str, expected_file_err: &str) -> Result<()> {
    let file_out = format_file_name(expected_file_out);
    let contents_out = fs::read_to_string(file_out.as_ref())?;
    let expected_out: Vec<&str> = contents_out.split('\n').filter(|s| !s.is_empty()).collect();

    let file_err = format_file_name(expected_file_err);
    let contents_err = fs::read_to_string(file_err.as_ref())?;
    let expected_err: Vec<&str> = contents_err.split('\n').filter(|s| !s.is_empty()).collect();

    let cmd = Command::v(PRG)?.args(args).assert();
    let out = cmd.get_output();
    let stdout = String::from_utf8(out.stdout.clone())?;
    let stderr = String::from_utf8(out.stderr.clone())?;
    let lines_out: Vec<&str> = stdout.split('\n').filter(|s| !s.is_empty()).collect();
    let lines_err: Vec<&str> = stderr.split('\n').filter(|s| !s.is_empty()).collect();

    assert_eq!(lines_out, expected_out);
    assert_eq!(lines_err, expected_err);

    Ok(())
}

// --------------------------------------------------
#[test]
fn storage_roots() -> Result<()> {
    run(
        &["list", "tests/cli/1.0", "tests/cli/1.1", "tests/cli/1.2"],
        "tests/cli/expected/storage_roots.out",
        "tests/cli/expected/storage_roots.err",
    )
}

//
// // --------------------------------------------------
// #[test]
// fn path1() -> Result<()> {
//     run(&["tests/cli/inputs"], "tests/cli/expected/path1.txt")
// }
//
// // --------------------------------------------------
// #[test]
// fn path_a() -> Result<()> {
//     run(&["tests/cli/inputs/a"], "tests/cli/expected/path_a.txt")
// }
//
// // --------------------------------------------------
// #[test]
// fn path_a_b() -> Result<()> {
//     run(&["tests/cli/inputs/a/b"], "tests/cli/expected/path_a_b.txt")
// }
//
// // --------------------------------------------------
// #[test]
// fn path_d() -> Result<()> {
//     run(&["tests/cli/inputs/d"], "tests/cli/expected/path_d.txt")
// }
//
// // --------------------------------------------------
// #[test]
// fn type_f() -> Result<()> {
//     run(&["tests/cli/inputs", "-t", "f"], "tests/cli/expected/type_f.txt")
// }
//
// // --------------------------------------------------
// #[test]
// fn type_f_path_a() -> Result<()> {
//     run(
//         &["tests/cli/inputs/a", "-t", "f"],
//         "tests/cli/expected/type_f_path_a.txt",
//     )
// }
//
// // --------------------------------------------------
// #[test]
// fn type_f_path_a_b() -> Result<()> {
//     run(
//         &["tests/cli/inputs/a/b", "--type", "f"],
//         "tests/cli/expected/type_f_path_a_b.txt",
//     )
// }
//
// // --------------------------------------------------
// #[test]
// fn type_f_path_d() -> Result<()> {
//     run(
//         &["tests/cli/inputs/d", "--type", "f"],
//         "tests/cli/expected/type_f_path_d.txt",
//     )
// }
//
// // --------------------------------------------------
// #[test]
// fn type_d() -> Result<()> {
//     run(&["tests/cli/inputs", "-t", "d"], "tests/cli/expected/type_d.txt")
// }
//
// // --------------------------------------------------
// #[test]
// fn type_d_path_a() -> Result<()> {
//     run(
//         &["tests/cli/inputs/a", "-t", "d"],
//         "tests/cli/expected/type_d_path_a.txt",
//     )
// }
//
// // --------------------------------------------------
// #[test]
// fn type_d_path_a_b() -> Result<()> {
//     run(
//         &["tests/cli/inputs/a/b", "--type", "d"],
//         "tests/cli/expected/type_d_path_a_b.txt",
//     )
// }
//
// // --------------------------------------------------
// #[test]
// fn type_d_path_d() -> Result<()> {
//     run(
//         &["tests/cli/inputs/d", "--type", "d"],
//         "tests/cli/expected/type_d_path_d.txt",
//     )
// }
//
// // --------------------------------------------------
// #[test]
// fn type_l() -> Result<()> {
//     run(&["tests/cli/inputs", "-t", "l"], "tests/cli/expected/type_l.txt")
// }
//
// // --------------------------------------------------
// #[test]
// fn type_f_l() -> Result<()> {
//     run(
//         &["tests/cli/inputs", "-t", "l", "f"],
//         "tests/cli/expected/type_f_l.txt",
//     )
// }
//
// // --------------------------------------------------
// #[test]
// fn name_csv() -> Result<()> {
//     run(
//         &["tests/cli/inputs", "-n", ".*[.]csv"],
//         "tests/cli/expected/name_csv.txt",
//     )
// }
//
// // --------------------------------------------------
// #[test]
// fn name_csv_mp3() -> Result<()> {
//     run(
//         &["tests/cli/inputs", "-n", ".*[.]csv", "-n", ".*[.]mp3"],
//         "tests/cli/expected/name_csv_mp3.txt",
//     )
// }
//
//
// // --------------------------------------------------
// #[test]
// fn name_a() -> Result<()> {
//     run(&["tests/cli/inputs", "-n", "a"], "tests/cli/expected/name_a.txt")
// }
//
// // --------------------------------------------------
// #[test]
// fn type_f_name_a() -> Result<()> {
//     run(
//         &["tests/cli/inputs", "-t", "f", "-n", "a"],
//         "tests/cli/expected/type_f_name_a.txt",
//     )
// }
//
// // --------------------------------------------------
// #[test]
// fn type_d_name_a() -> Result<()> {
//     run(
//         &["tests/cli/inputs", "--type", "d", "--name", "a"],
//         "tests/cli/expected/type_d_name_a.txt",
//     )
// }
//
// // --------------------------------------------------
// #[test]
// fn path_g() -> Result<()> {
//     run(&["tests/cli/inputs/g.csv"], "tests/cli/expected/path_g.txt")
// }
//
// // --------------------------------------------------
// #[test]
// #[cfg(not(windows))]
// fn unreadable_dir() -> Result<()> {
//     let dirname = "tests/cli/inputs/cant-touch-this";
//     if !Path::new(dirname).exists() {
//         fs::create_dir(dirname)?;
//     }
//
//     std::process::Command::new("chmod")
//         .args(["000", dirname])
//         .status()
//         .expect("failed");
//
//     let cmd = Command::cargo_bin(PRG)?
//         .arg("tests/cli/inputs")
//         .assert()
//         .success();
//     fs::remove_dir(dirname)?;
//
//     let out = cmd.get_output();
//     let stdout = String::from_utf8(out.stdout.clone())?;
//     let lines: Vec<&str> =
//         stdout.split('\n').filter(|s| !s.is_empty()).collect();
//
//     assert_eq!(lines.len(), 17);
//
//     let stderr = String::from_utf8(out.stderr.clone())?;
//     assert!(stderr.contains("cant-touch-this: Permission denied"));
//     Ok(())
// }

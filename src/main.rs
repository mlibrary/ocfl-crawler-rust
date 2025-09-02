use anyhow::{anyhow, Result};
use clap::{builder::PossibleValue, ArgAction, Parser, Subcommand, ValueEnum};
use ocfl_crawler_rust::{get_object_id, is_object_root, is_storage_root, DirGuard};
use regex::Regex;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

#[derive(Debug, Parser)]
#[command(author, version, about, propagate_version = true)]
/// OCFL crawler in Rust
struct Cli {
    /// Command to execute
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// List OCFL objects under one or more storage roots
    List(ListCmd),
    /// Show info for a single OCFL object root
    Info(InfoCmd),
}

#[derive(Debug, clap::Args)]
struct ListCmd {
    /// OCFL Storage Root path(s)
    #[arg(value_name = "PATH", default_value = ".")]
    paths: Vec<String>,

    /// Names
    #[arg(
        short('n'),
        long("name"),
        value_name = "NAME",
        value_parser(Regex::new),
        action(ArgAction::Append),
        num_args(0..)
    )]
    names: Vec<Regex>,

    /// Entry types
    #[arg(
        short('t'),
        long("type"),
        value_name = "TYPE",
        value_parser(clap::value_parser!(EntryType)),
        action(ArgAction::Append),
        num_args(0..)
    )]
    entry_types: Vec<EntryType>,
}

#[derive(Debug, clap::Args)]
struct InfoCmd {
    /// Path to an OCFL object root (directory containing inventory.json)
    #[arg(value_name = "PATH")]
    path: String,
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum EntryType {
    Dir,
    File,
    Link,
}

impl ValueEnum for EntryType {
    fn value_variants<'a>() -> &'a [Self] {
        &[EntryType::Dir, EntryType::File, EntryType::Link]
    }

    fn to_possible_value<'a>(&self) -> Option<PossibleValue> {
        Some(match self {
            EntryType::Dir => PossibleValue::new("d"),
            EntryType::File => PossibleValue::new("f"),
            EntryType::Link => PossibleValue::new("l"),
        })
    }
}

// --------------------------------------------------
fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Command::List(args) => run_list(args),
        Command::Info(args) => run_info(args),
    };

    if let Err(e) = result {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

// --------------------------------------------------
fn run_list(args: ListCmd) -> Result<()> {
    let _type_filter = |entry: &DirEntry| {
        args.entry_types.is_empty()
            || args.entry_types.iter().any(|entry_type| match entry_type {
            EntryType::Link => entry.file_type().is_symlink(),
            EntryType::Dir => entry.file_type().is_dir(),
            EntryType::File => entry.file_type().is_file(),
        })
    };

    let _name_filter = |entry: &DirEntry| {
        args.names.is_empty()
            || args
            .names
            .iter()
            .any(|re| re.is_match(&entry.file_name().to_string_lossy()))
    };

    let object_filter = |entry: &DirEntry| is_object_root(entry.path());

    for path in &args.paths {
        if is_storage_root(path) {
            let _guard = DirGuard::change_to(path)?;

            let entries = WalkDir::new(".")
                .min_depth(1)
                .into_iter()
                .filter_map(|e| match e {
                    Err(e) => {
                        eprintln!("{e}");
                        None
                    }
                    Ok(entry) => Some(entry),
                })
                .filter(object_filter)
                .map(|entry| object_to_json(entry.path().display().to_string()))
                .collect::<Vec<_>>();
            for entry in &entries {
                println!("{}", entry);
            }
        } else {
            let abs_path = Path::new(path).canonicalize().unwrap();
            let path_str = abs_path.display().to_string();
            eprintln!("{path_str} is not a storage root");
        }
    }

    Ok(())
}

fn run_info(args: InfoCmd) -> Result<()> {
    let p = Path::new(&args.path);
    if !is_object_root(p) {
        let abs = p.canonicalize().unwrap_or_else(|_| p.to_path_buf());
        return Err(anyhow!(format!(
            "{} is not an OCFL object root",
            abs.display()
        )));
    }

    // Use current directory as "storage" context for the output
    println!("{}", object_to_json(p));
    Ok(())
}

pub fn object_to_json<P: AsRef<Path>>(path: P) -> String {
    // path.as_ref().display().to_string()
    // let rel_path = path.as_ref().display().to_string();
    let path_abs = path
        .as_ref()
        .canonicalize()
        .unwrap()
        .display()
        .to_string();
    // let path_str = abs_path.display().to_string();
    // let _rel_root = root.as_ref().display().to_string();
    // let abs_root = root.as_ref().canonicalize().unwrap();
    // let root_str = abs_root.display().to_string();
    // let cwd_abs = std::fs::canonicalize(std::env::current_dir().unwrap())
    //     .unwrap()
    //     .display()
    //     .to_string();
    let id_str = get_object_id(path).unwrap_or_else(|_| String::from(""));
    String::from(format!(
        "{{ \"object_root\": \"{path_abs}\", \"id\": \"{id_str}\" }}"
    ))
}

// // Usage
// fn main() -> io::Result<()> {
//     let before = env::current_dir()?;
//     println!("Before: {}", before.display());
//
//     {
//         let _guard = DirGuard::change_to("/tmp")?;
//         // CWD is now /tmp
//         println!("Inside: {}", env::current_dir()?.display());
//         // When `_guard` goes out of scope, CWD is restored automatically.
//     }
//
//     println!("After: {}", env::current_dir()?.display());
//     Ok(())
// }
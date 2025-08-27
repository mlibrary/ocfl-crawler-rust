use anyhow::Result;
use clap::{builder::PossibleValue, ArgAction, Parser, ValueEnum};
use ocfl_crawler_rust::{is_object_root, is_storage_root};
use regex::Regex;
use walkdir::{DirEntry, WalkDir};

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// OCFL crawler in Rust
struct Args {
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
    if let Err(e) = run(Args::parse()) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

// --------------------------------------------------
fn run(args: Args) -> Result<()> {
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

    let object_filter = |entry: &DirEntry| {
        is_object_root(entry.path())
    };

    for path in &args.paths {
        if is_storage_root(path) {
            println!("storage root: {path}");
            let entries = WalkDir::new(path)
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
                .map(|entry| entry.path().display().to_string())
                .collect::<Vec<_>>();
            for entry in &entries {
                println!("object root: {}", entry);
            }
        } else {
            eprintln!("{path} is not a storage root");
        }
    }

    Ok(())
}
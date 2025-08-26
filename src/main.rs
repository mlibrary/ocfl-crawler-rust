use anyhow::Result;
use clap::{builder::PossibleValue, ArgAction, Parser, ValueEnum};
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
    let type_dir_filter = |entry: &DirEntry| {
        entry.file_type().is_dir()
    };

    let type_file_filter = |entry: &DirEntry| {
        entry.file_type().is_file()
    };

    let type_filter = |entry: &DirEntry| {
        args.entry_types.is_empty()
            || args.entry_types.iter().any(|entry_type| match entry_type {
            EntryType::Link => entry.file_type().is_symlink(),
            EntryType::Dir => entry.file_type().is_dir(),
            EntryType::File => entry.file_type().is_file(),
        })
    };

    let storage_root_version_filter = |entry: &DirEntry| {
        [Regex::new("0=ocfl_1.0").unwrap(), Regex::new("0=ocfl_1.1").unwrap()].iter().any(|re| re.is_match(&entry.file_name().to_string_lossy()))
    };

    let name_filter = |entry: &DirEntry| {
        args.names.is_empty()
            || args
            .names
            .iter()
            .any(|re| re.is_match(&entry.file_name().to_string_lossy()))
    };

    for path in &args.paths {
        let storage_root_path = WalkDir::new(path)
            .max_depth(0)
            .into_iter()
            .filter_map(|e| match e {
                Err(e) => {
                    eprintln!("{e}");
                    None
                }
                Ok(entry) => Some(entry),
            })
            .filter(type_dir_filter)
            .map(|entry| entry.path().display().to_string())
            .collect::<Vec<_>>().join("\n");

        let storage_root_version = WalkDir::new(path)
            .min_depth(1)
            .max_depth(1)
            .into_iter()
            .filter_map(|e| match e {
                Err(e) => {
                    eprintln!("{e}");
                    None
                }
                Ok(entry) => Some(entry),
            })
            .filter(type_file_filter)
            .filter(storage_root_version_filter)
            .map(|entry| entry.file_name().display().to_string())
            .collect::<Vec<_>>().join("\n");

        let object_root_paths = WalkDir::new(path)
            .min_depth(1)
            .max_depth(1)
            .into_iter()
            .filter_map(|e| match e {
                Err(e) => {
                    eprintln!("{e}");
                    None
                }
                Ok(entry) => Some(entry),
            })
            .filter(type_dir_filter)
            .map(|entry| format!("{{ id: y, path: {}, version: x }}", entry.path().display()))
            .collect::<Vec<_>>().join(", ");

        // let object_root_version = WalkDir::new(path)
        //     .min_depth(2)
        //     .max_depth(2)
        //     .into_iter()
        //     .filter_map(|e| match e {
        //         Err(e) => {
        //             eprintln!("{e}");
        //             None
        //         }
        //         Ok(entry) => Some(entry),
        //     })
        //     .filter(type_file_filter)
        //     .filter(object_root_version_filter)
        //     .map(|entry| entry.file_name().display().to_string())
        //     .collect::<Vec<_>>().join("\n");

        println!("{{ storage: {storage_root_path}, version: {storage_root_version}, objects: [{object_root_paths}] }}");
    }

    // for path in &args.paths {
    //     let entries = WalkDir::new(path)
    //         .into_iter()
    //         .filter_map(|e| match e {
    //             Err(e) => {
    //                 eprintln!("{e}");
    //                 None
    //             }
    //             Ok(entry) => Some(entry),
    //         })
    //         .filter(type_filter)
    //         .filter(name_filter)
    //         .map(|entry| entry.path().display().to_string())
    //         .collect::<Vec<_>>();
    //
    //     println!("{}", entries.join("\n"));
    // }

    Ok(())
}
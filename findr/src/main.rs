use anyhow;
use anyhow::Result;
use clap::builder::PossibleValue;
use clap::{ArgAction, Parser, ValueEnum};
use regex::Regex;
use walkdir::{DirEntry, WalkDir};

fn main() {
    let args = Args::parse();
    if let Err(err) = run(args) {
        eprintln!("{err}");
        std::process::exit(1);
    };
}

fn run(args: Args) -> Result<()> {
    get_paths(&args.paths, &args.entry_types)?;
    // println!("{args:#?}");
    Ok(())
}

fn get_paths(paths: &Vec<String>, entry_types: &Vec<EntryType>) -> Result<()> {
    let mut ok: Result<()> = Ok(());
    for path in paths {
        for possible_entry in WalkDir::new(path) {
            match possible_entry {
                Ok(entry) => print_path(entry, &entry_types),
                Err(err) => {
                    ok = Err(anyhow::anyhow!(err));
                }
            }
        }
    }
    ok
}

fn print_path(entry: DirEntry, entry_types: &Vec<EntryType>) {
    let file_type = entry.file_type();
    let print_all = entry_types.len() == 0;
    if file_type.is_file() && (entry_types.contains(&EntryType::File) || print_all) {
        println!("{}", entry.path().display());
    } else if file_type.is_dir() && (entry_types.contains(&EntryType::Dir) || print_all) {
        println!("{}", entry.path().display());
    } else if file_type.is_symlink() && (entry_types.contains(&EntryType::Link) || print_all) {
        println!("{}", entry.path().display());
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Search paths
    #[arg(value_name = "PATH", default_value = ".")]
    paths: Vec<String>,

    /// Name
    #[arg(
        short = 'n',
        long = "name",
        value_name = "NAME",
        value_parser(Regex::new),
        action(ArgAction::Append),
        num_args(0..)
    )]
    names: Vec<Regex>,

    // Entry type
    #[arg(short = 't', long = "type", value_name = "TYPE", action(ArgAction::Append), num_args(0..))]
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
    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            EntryType::Dir => PossibleValue::new("d"),
            EntryType::File => PossibleValue::new("f"),
            EntryType::Link => PossibleValue::new("l"),
        })
    }
}

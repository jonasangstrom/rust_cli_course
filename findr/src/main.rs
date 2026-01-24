use anyhow::Result;
use clap::builder::PossibleValue;
use clap::{ArgAction, Parser, ValueEnum};
use regex::Regex;
use walkdir::WalkDir;

fn main() {
    let args = Args::parse();
    if let Err(err) = run(args) {
        eprintln!("{err}");
        std::process::exit(1);
    };
}

fn run(args: Args) -> Result<()> {
    get_paths(&args.paths);
    // println!("{args:#?}");
    Ok(())
}

fn get_paths(paths: &Vec<String>) -> Result<()> {
    for path in paths {
        for entry in WalkDir::new(path) {
            match entry {
                Err(e) => eprintln!("e"),
                Ok(entry) => println!("{}", entry.path().display()),
            }
        }
    }
    Ok(())
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

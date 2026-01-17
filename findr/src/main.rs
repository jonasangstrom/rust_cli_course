use anyhow::Result;
use clap::builder::PossibleValue;
use clap::{Parser, ValueEnum};
use regex::Regex;

fn main() {
    let mut args = Args::parse();
    println!("{args:#?}");
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Search paths
    #[arg(value_name = "PATH", default_value = ".")]
    paths: Vec<String>,

    /// Name
    #[arg(short, long, value_name = "NAME")]
    name: Vec<Regex>,

    // Entry type
    #[arg(short = 't', long = "type", value_name = "TYPE")]
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

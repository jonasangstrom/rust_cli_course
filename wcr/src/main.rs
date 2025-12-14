use clap::Parser;
use std::io::{self, BufRead, BufReader};
use anyhow::Result;

fn main() {
    let mut args = Args::parse();
    // this is to make it behave like wc default
    if [args.words, args.bytes, args.chars, args.lines].iter().all(|v| v == &false) {
        args.words = true;
        args.chars = true;
        args.bytes = true;
    }
    println!("{args:?}");
}

fn open(filename: &String) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(File::open(filename)?))
    }
}

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of wc
struct Args {
    /// Input file(s)
    #[arg(
        value_name="FILE",
        default_value="-"
    ) ]
    files: Vec<String>,

    /// Show line count
    #[arg(short, long)]
    lines: bool,

    /// Show word count
    #[arg(short, long)]
    words: bool,

    /// Show bytes count
    #[arg(short('c'), long)]
    bytes: bool,

    /// Show chars count
    #[arg(short('m'), long, conflicts_with("bytes"))]
    chars: bool
}

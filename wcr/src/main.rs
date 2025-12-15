use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() {
    let mut args = Args::parse();
    // this is to make it behave like wc default
    if [args.words, args.bytes, args.chars, args.lines]
        .iter()
        .all(|v| v == &false)
    {
        args.lines = true;
        args.words = true;
        args.bytes = true;
    }

    if let Err(err) = run(args) {
        eprintln!("{err}");
        std::process::exit(1);
    };
}

fn run(args: Args) -> Result<()> {
    for filename in args.files {
        let buffer = open(&filename)?;
        let counts = count_things(buffer)?;
        let result_line = make_result_line(counts, &filename);
        println!("{result_line}");
    }
    Ok(())
}

#[derive(Debug)]
struct Counts {
    words: Option<u32>,
    lines: Option<u32>,
    bytes_or_chars: Option<u32>,
}

fn make_result_line(counts: Counts, filename: &str) -> String {
    format!(
        "{}{}{}{}",
        format_field(counts.words),
        format_field(counts.lines),
        format_field(counts.bytes_or_chars),
        match filename {
            "-" => "".to_string(),
            name => format!(" {name}"),
        }
    )
}

fn format_field(optional_value: Option<u32>) -> String {
    match optional_value {
        Some(value) => format!("{value:>8}"),
        None => "".to_string(),
    }
}

fn count_things(buffer: Box<dyn BufRead>) -> Result<Counts> {
    let mut line_count: u32 = 0;
    let mut char_count: u32 = 0;
    let mut words_count: u32 = 0;
    let mut in_word: bool;
    for possible_line in buffer.lines() {
        let line = possible_line?;
        in_word = false;
        for char in line.chars() {
            char_count += 1;
            if char.is_whitespace() {
                if in_word {
                    words_count += 1;
                    in_word = false
                }
            } else {
                in_word = true;
            }
        }
        if in_word {
            words_count += 1;
        }
        line_count += 1;
    }
    Ok(Counts {
        words: Some(words_count),
        lines: Some(line_count),
        bytes_or_chars: Some(char_count),
    })
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of wc
struct Args {
    /// Input file(s)
    #[arg(value_name = "FILE", default_value = "-")]
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
    chars: bool,
}

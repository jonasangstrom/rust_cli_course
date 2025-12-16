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

struct ShallCount {
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

#[derive(Debug)]
struct Counts {
    words: Option<u32>,
    lines: Option<u32>,
    bytes_or_chars: Option<u32>,
}

fn run(args: Args) -> Result<()> {
    let shall_count = ShallCount {
        lines: args.lines,
        words: args.words,
        bytes: args.bytes,
        chars: args.chars,
    };

    let mut total_lines = 0;
    let mut total_words = 0;
    let mut total_bytes_or_chars = 0;
    let more_than_one_file = args.files.len() > 1;
    for filename in args.files {
        let buffer = match open(&filename) {
            Ok(buffer) => buffer,
            Err(err) => {
                eprint!("{filename}: ");
                return Err(err);
            }
        };
        let counts = count_things(buffer, &shall_count)?;
        if let Some(lines) = counts.lines {
            total_lines += lines;
        }
        if let Some(words) = counts.words {
            total_words += words;
        }
        if let Some(bytes_or_chars) = counts.bytes_or_chars {
            total_bytes_or_chars += bytes_or_chars;
        }
        let result_line = make_result_line(counts, &filename);
        println!("{result_line}");
    }
    if more_than_one_file {
        let total_counts = Counts {
            words: match shall_count.words {
                true => Some(total_words),
                false => None,
            },
            lines: match shall_count.lines {
                true => Some(total_lines),
                false => None,
            },
            bytes_or_chars: match shall_count.bytes || shall_count.chars {
                true => Some(total_bytes_or_chars),
                false => None,
            },
        };
        let result_line = make_result_line(total_counts, "total");
        println!("{result_line}");
    }
    Ok(())
}

fn make_result_line(counts: Counts, filename: &str) -> String {
    format!(
        "{}{}{}{}",
        format_field(counts.lines),
        format_field(counts.words),
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

fn count_things(mut buffer: Box<dyn BufRead>, shall_count: &ShallCount) -> Result<Counts> {
    let mut line_count: u32 = 0;
    let mut byte_or_char_count: u32 = 0;
    let mut words_count: u32 = 0;
    let mut in_word: bool;
    let mut read = true;
    let mut line = String::new();
    while read {
        let bytes = buffer.read_line(&mut line)?;
        if bytes == 0 {
            read = false;
            continue;
        }
        if shall_count.lines {
            line_count += 1;
        }
        if shall_count.bytes {
            byte_or_char_count += u32::try_from(bytes)?;
        }
        in_word = false;

        for char in line.chars() {
            if shall_count.chars {
                byte_or_char_count += 1;
            }
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
        line.clear();
    }
    Ok(Counts {
        words: match shall_count.words {
            true => Some(words_count),
            false => None,
        },
        lines: match shall_count.lines {
            true => Some(line_count),
            false => None,
        },
        bytes_or_chars: match shall_count.bytes || shall_count.chars {
            true => Some(byte_or_char_count),
            false => None,
        },
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

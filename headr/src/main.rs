use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use anyhow::Result;

fn main() {
    let args = Args::parse();
    for filename in args.files {
        let buffer = open(&filename);
        match buffer {
            Err(err) => eprintln!("Failed to read file {err}"),
            Ok(file) => print_to_stout(file),
        }
    }
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn print_to_stout(buffer_box: Box<dyn BufRead>) {
    for line_result  in buffer_box.lines() {
        match line_result {
            Err(err) => eprintln!("Failed to read line {err}"),
            Ok(line) => println!("{line}"),
        }
    }
}

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// "Rust version of head"
struct Args {
    #[arg(value_name = "FILE", default_value="-")]
    files: Vec<String>,
    #[arg(
        short('n'),
        long("number-lines"),
        conflicts_with("number_nonblank_lines")
    )]
    number_lines: bool,
    #[arg(short('b'), long("number-nonblank"))]
    number_nonblank_lines: bool,
}

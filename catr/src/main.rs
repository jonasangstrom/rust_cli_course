use clap::Parser;
use anyhow::Result;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() {
    let args = Args::parse();
    if let Err(e) = run(args) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn run(args: Args) -> Result<()> {
    let line_numbering: LineNumbering;
    if args.number_nonblank_lines {
        line_numbering = LineNumbering::Nonblank;
    }
    else if args.number_lines {
        line_numbering = LineNumbering::Yes;
    }
    else {
        line_numbering = LineNumbering::No;
    }
    let mut line_number = 1;
    
    for filename in args.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {filename}: {err}"),
            Ok(buffer_box) => line_number = print_to_stout(buffer_box, &line_numbering, &line_number),
        }
    }
    Ok(())
}

enum LineNumbering {
    Yes,
    Nonblank,
    No
}

fn print_to_stout(buffer_box: Box<dyn BufRead>, line_numbering: &LineNumbering, line_number: &i32) -> i32 {
    let mut new_line_number: i32 = *line_number;
    for line_result  in buffer_box.lines() {
        match line_result {
            Err(err) => eprintln!("Failed to read line {err}"),
            Ok(line) => new_line_number = print_line(line, &new_line_number, &line_numbering),
        }
    }
    new_line_number
}

fn print_line(line: String, line_number: &i32, line_numbering: &LineNumbering) -> i32 {
    let return_number: i32;
    match line_numbering {
        LineNumbering::No => {
            println!("{line}");
            return_number = 1;
        },
        LineNumbering::Yes => {
            println!("{line_number:>6}\t{line}");
            return_number = line_number + 1;
        },
        LineNumbering::Nonblank => {
            if line == "" {
                return_number = line_number + 0;
                println!("{line}");
            }
            else {
                println!("{line_number:>6}\t{line}");
                return_number = line_number + 1;
            }
        }
    };
    
    return return_number;
}

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// "Rust version of cat"
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

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

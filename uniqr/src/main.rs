use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() {
    let args = Args::parse();
    if let Err(err) = run(args) {
        eprintln!("{err}");
        std::process::exit(1);
    };
}

fn run(args: Args) -> Result<()> {
    let filename = args.in_file;
    let buffer = match open(&filename) {
        Ok(buffer) => buffer,
        Err(err) => {
            eprint!("{filename}: ");
            return Err(err);
        }
    };
    read_file(buffer, args.count)?;
    Ok(())
}

fn read_file(mut buffer: Box<dyn BufRead>, print_line_number: bool) -> Result<()> {
    let mut line = String::new();
    let mut old_line = String::new();
    let mut first = true;
    let mut optional_line_number: Option<i32> = match print_line_number {
        true => Some(0),
        false => None,
    };
    loop {
        optional_line_number = match optional_line_number {
            Some(line_number) => Some(line_number + 1),
            None => None,
        };
        let bytes = buffer.read_line(&mut line)?;
        if bytes == 0 {
            if !first {
                write_output(&old_line, &optional_line_number);
            }
            break;
        }

        if old_line.trim_end() != line.trim_end() {
            if first {
                first = false;
            } else {
                write_output(&old_line, &optional_line_number);
            }
            old_line = format!("{line}");
            optional_line_number = match optional_line_number {
                Some(_) => Some(0),
                None => None,
            };
        } else {
        }
        line.clear();
    }
    Ok(())
}

fn write_output(line: &String, optional_line_number: &Option<i32>) {
    match optional_line_number {
        Some(line_number) => print!("{line_number:>4} {line}"),
        None => print!("{line}"),
    };
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Input file
    #[arg(value_name = "IN_FILE", default_value = "-")]
    in_file: String,

    /// Output file
    #[arg(value_name = "OUT_FILE")]
    out_file: Option<String>,

    /// Show counts
    #[arg(short, long)]
    count: bool,
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

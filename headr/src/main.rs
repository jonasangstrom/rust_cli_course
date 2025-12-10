use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use anyhow::Result;

fn main() {
    let args = Args::parse();
    let number_of_lines = args.lines;
    let filenames = args.files;
    let print_filename = match filenames.len()
    {
        1 => false,
        _ => true,
    };
    for filename in filenames {
        if print_filename
        {
            println!("==> {filename} <==\n");
        }
        let buffer = open(&filename);
        match buffer {
            Err(err) => eprintln!("Failed to read file {err}"),
            Ok(file) => print_to_stout( file, number_of_lines as usize),
        }
    }
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn print_to_stout(buffer_box: Box<dyn BufRead>, number_of_lines: usize) {

    for (line_number, line_result)  in buffer_box.lines().enumerate() {
        match line_result {
            Err(err) => eprintln!("Failed to read line {err}"),
            Ok(line) => println!("{line}"),
        }
        if (line_number + 1) >= number_of_lines {
            break;
        }
    }
}

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// "Rust version of head"
struct Args {
    #[arg(
        value_name ="FILE",
        help="Input file(s)",
        default_value="-")
    ]
    files: Vec<String>,
    #[arg(
        value_name ="LINES",
        short('n'),
        help="Number of lines",
        long("lines"),
        value_parser=clap::value_parser!(u64).range(1..),
        default_value="10",
    )]
    lines: u64,
    #[arg(
        short('c'),
        value_name ="BYTES",
        help="Number of bytes",
        long("bytes"),
        conflicts_with("lines"),
        value_parser=clap::value_parser!(u64).range(1..),
    )]
    bytes: Option<u64>,
}

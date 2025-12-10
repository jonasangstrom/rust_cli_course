use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use anyhow::Result;

fn main() {
    run().unwrap();
}

fn run() -> Result<()> {
    let args = Args::parse();
    let number_of_lines = args.lines as usize;
    let filenames = args.files;
    let n_files = filenames.len();
    let print_filename = match filenames.len()
    {
        1 => false,
        _ => true,
    };
    for (n_file, filename) in filenames.iter().enumerate(){
        if print_filename
        {
            print!("==> {filename} <==\n");
        }
        let buffer = open(&filename);
        match buffer {
            Err(err) => eprintln!("Failed to read file {err}"),
            Ok(file) => print_file_to_stout_lines( file, &number_of_lines)?,
        }
        let last_file = (n_file + 1) != n_files;
        if print_filename & last_file
        {
            print!("\n");
        }
    }
    Ok(())
}


fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn print_file_to_stout_lines(mut file: Box<dyn BufRead>, number_of_lines: &usize) -> Result<()> {
    let mut line =  String::new();
    for _line_number in 0..*number_of_lines {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }
        print!("{line}");
        line.clear();
    }
    Ok(())
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

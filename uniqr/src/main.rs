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
    read_file(buffer)?;
    Ok(())
}

fn read_file(mut buffer: Box<dyn BufRead>) -> Result<()> {
    let mut line = String::new();
    let mut old_line = String::new();
    loop {
        let bytes = buffer.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }
        if old_line == line {
            line.clear();
            continue;
        }
        print!("{line}");
        old_line.clear();
        old_line = format!("{line}");
        line.clear();
    }
    Ok(())
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

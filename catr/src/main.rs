use clap::Parser;
use anyhow::Result;

fn main() {
    let args = Args::parse();
    if let Err(e) = run(args) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn run(args: Args) -> Result<()> {
    for filename in args.files {
        println!("{filename}");
    }
    Ok(())
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

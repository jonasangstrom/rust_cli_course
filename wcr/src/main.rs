use clap::Parser;
fn main() {
    let args = Args::parse();
    println!("{args:?}");
}

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(
        value_name="FILE",
        help="Input file(s)",
        default_value="-"
    ) ]
    files: Vec<String>,
    #[arg(short, long)]
    lines: bool,
    #[arg(short, long)]
    words: bool,
    #[arg(short('c'), long)]
    bytes: bool,
    #[arg(short('m'), long, conflicts_with("bytes"))]
    chars: bool
}

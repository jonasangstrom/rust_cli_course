use clap::Parser;
fn main() {
    let args = Args::parse();
    println!("{args:?}");
}

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of wc
struct Args {
    /// Input file(s)
    #[arg(
        value_name="FILE",
        default_value="-"
    ) ]
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
    chars: bool
}

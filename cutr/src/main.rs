use clap::Parser;

fn main() {
    println!("Hello, world!");
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
/// Rust version of 'cut'
struct Args {
    /// Input file(s)
    #[arg(default_value = "-")]
    files: Vec<String>,
    /// Field delimiter
    #[arg(short, long, value_name = "DELIMITER", default_value = "\t")]
    delimiter: String,
    /// Selected fields
    #[command(flatten)]
    extract: ArgsExtract,
}
#[derive(Debug, clap::Args)]
#[group(required = true, multiple = false)]
struct ArgsExtract {
    ///Selected fields
    #[arg(short, long, value_name = "FIELDS")]
    fields: Option<String>,
    ///Selected bytes
    #[arg(short, long, value_name = "BYTES")]
    bytes: Option<String>,
    ///Selected chars
    #[arg(short, long, value_name = "CHARS")]
    chars: Option<String>,
}

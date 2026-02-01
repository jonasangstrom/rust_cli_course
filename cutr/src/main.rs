use anyhow::{Result, bail};
use clap::Parser;

fn main() {
    let mut args = Args::parse();
    if let Err(err) = run(args) {
        eprintln!("{err}");
        std::process::exit(1);
    }
}

fn run(args: Args) -> Result<()> {
    let delimiter = create_delimiter(&args.delimiter)?;
    println!("{args:?}");
    Ok(())
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

fn create_delimiter(string_delimiter: &String) -> Result<u8> {
    let bytes_delimiter = string_delimiter.as_bytes();
    match bytes_delimiter.len() {
        1 => Ok(*bytes_delimiter.first().unwrap()),
        _ => bail!(r#"--delim "{string_delimiter}" must be a single byte"#),
    }
}

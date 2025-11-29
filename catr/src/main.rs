use clap::Parser;

fn main() {
    let args = Args::parse();
    println!("{:#?}", args);
}

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// "Rust version of cat"
struct Args {
    #[arg(required(true))]
    files: Vec<String>,
    #[arg(short('n'))]
    number: bool,
    #[arg(short('b'))]
    #[arg(long("number-nonblank"))]
    number_nonblank: bool,
}

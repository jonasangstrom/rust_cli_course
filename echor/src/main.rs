use clap::Parser;

fn main() {
    let args = Args::parse();

    let ending: &str = if args.omit_newline {""} else {"\n"};

    print!("{}{ending}", args.text.join(" "));
}

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// "Rust version of echo"
struct Args {
    #[arg(required(true))]
    text: Vec<String>,
    #[arg(short('n'))]
    omit_newline: bool,
}

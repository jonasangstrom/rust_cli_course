use clap::{Command, Arg, ArgAction};

fn main() {
    let matches = Command::new("echor")
        .version("0.1.0")
        .author("Jonas Ångström")
        .about("Rust version of echo")
        .arg(
            Arg::new("text")
            .value_name("TEXT")
            .required(true)
            .num_args(1..)
        )
        .arg(
            Arg::new("omit_newline")
            .short('n')
            .action(ArgAction::SetTrue)
            .help("Do not print newline.")
        )
        .get_matches();

    let omit_newline = matches.get_flag("omit_newline");
    let ending: &str = if omit_newline {""} else {"\n"};

    let text: Vec<String> = matches.get_many("text").unwrap().cloned().collect();

    print!("{}{ending}", text.join(" "));
}


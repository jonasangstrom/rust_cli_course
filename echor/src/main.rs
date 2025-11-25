use clap::Command;

fn main() {
    let _matches = Command::new("echor")
        .version("0.1.0")
        .author("Jonas Ångström")
        .about("Rust version of echo")
        .get_matches();

    println!("{:?}", _matches);
}

use clap::{arg, Command};

fn main() {
    let matches = Command::new("ugsr")
        .version("0.1.0")
        .author("Kevin K. <kbknapp@gmail.com>")
        .about("rust version of UGS CLI")
        .arg(arg!(--two <VALUE>).required(true))
        .arg(arg!(--one <VALUE>).required(true))
        .get_matches();
    println!(
        "two: {:?}",
        matches.get_one::<String>("two").expect("required")
    );
    println!(
        "one: {:?}",
        matches.get_one::<String>("one").expect("required")
    );
}
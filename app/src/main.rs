use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("[Error]: {}", err);
        process::exit(1);
    });

}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args.get(1).expect("Please enter query argument").clone();
        let filename = args.get(2).expect("Please enter file path argument").clone();

        Ok(Config { query, filename })
    }
}
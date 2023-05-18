use std::str;
use clap::{Parser, Subcommand};
use encoding::{all::UTF_8, Encoding};
use base64::{engine::general_purpose, Engine as _};
use config::Config;

#[derive(Parser)]
#[command(version, about = "rust version of Unity Gaming Services CLI")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// store service account 'key-id' and 'secret-key' to access UGS
    Login {
        #[arg(short, long, value_name = "Key ID")]
        key_id: String,
    
        #[arg(short, long, value_name = "Secret Key")]
        secret_key: String,
    },
}

fn main() {
    //let settings = Config::builder().add_source(config::File::with_name("UnityServices/credential")).build().unwrap();
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Login { key_id, secret_key }) => {
            let encode_result = UTF_8.encode(&format!("{}:{}", key_id, secret_key), encoding::EncoderTrap::Ignore).unwrap();
            let token = general_purpose::STANDARD.encode(encode_result);
            println!("{}", token);
        }
        None => {}
    }
}
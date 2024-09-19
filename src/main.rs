use std::path::PathBuf;

use clap::{Parser, Subcommand};
use totp_rs::{Rfc6238, Secret, TOTP};

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    #[arg(short, long, default_value = ".secret")]
    secret: PathBuf,
}

#[derive(Debug, Subcommand, Clone)]
enum Commands {
    Generate,
}

fn load_secret(secret: PathBuf) -> Vec<u8> {
    let secret_str = std::fs::read_to_string(secret).expect("Unable to read file");
    Secret::Encoded(secret_str)
        .to_bytes()
        .expect("unable to decode secret")
}

fn generate_secret(args: Cli) {
    let str = Secret::generate_secret().to_encoded().to_string();
    std::fs::write(&args.secret, str).expect("Unable to write file");
    println!("Generated secret");
}

fn default_command(args: Cli) {
    let secret = load_secret(args.secret);

    let rfc = Rfc6238::with_defaults(secret).expect("Unable to parse secret");
    let totp = TOTP::from_rfc6238(rfc).unwrap();
    let token = totp.generate_current().expect("Unable to generate code");
    let ttl = totp.ttl().unwrap();
    // let seconds_remaining = totp.

    println!("{} ({})", token, ttl);
}

fn main() {
    let args = Cli::parse();

    match args.command {
        None => default_command(args),
        Some(Commands::Generate) => generate_secret(args),
    }
}

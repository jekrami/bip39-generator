use bip39_generator_lib::{generate_mnemonic_phrase, save_phrases, validate_mnemonic_phrase};
use clap::Parser;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 100)]
    count: usize,

    #[arg(short, long, num_args = 0..=1, require_equals = true, default_missing_value = "default")]
    output: Option<String>,

    #[arg(short, long)]
    validate: Option<String>,
}

fn main() {
    let args = Args::parse();

    if let Some(phrase_to_validate) = args.validate {
        if validate_mnemonic_phrase(&phrase_to_validate) {
            println!("Mnemonic is valid.");
        } else {
            println!("Mnemonic is not valid.");
        }
        return;
    }

    let mut phrases = Vec::new();
    for _ in 0..args.count {
        phrases.push(generate_mnemonic_phrase());
    }

    if let Some(output_file) = args.output {
        let filename = if output_file == "default" {
            let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
            format!("mnemonics_{}.txt", timestamp)
        } else {
            output_file
        };

        if let Err(e) = save_phrases(&phrases, &filename) {
            eprintln!("Error saving phrases: {}", e);
        } else {
            println!("Successfully saved {} mnemonics to {}", args.count, filename);
        }
    } else {
        for phrase in phrases {
            println!("{}", phrase);
        }
    }
}

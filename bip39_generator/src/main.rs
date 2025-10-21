use rand::{RngCore, rngs::OsRng};
use sha2::{Sha256, Digest};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

const ENTROPY_BITS: usize = 128;
const CHECKSUM_BITS: usize = ENTROPY_BITS / 32;
const MNEMONIC_WORDS: usize = (ENTROPY_BITS + CHECKSUM_BITS) / 11;

use clap::Parser;
use std::io::Write;

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

use std::env;

fn main() {
    let args = Args::parse();

    let wordlist_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("bip39-english.txt");

    let wordlist = match read_wordlist(&wordlist_path) {
        Ok(words) => words,
        Err(e) => {
            eprintln!(
                "Error reading wordlist at '{}': {}",
                wordlist_path.display(),
                e
            );
            return;
        }
    };

    if let Some(phrase_to_validate) = args.validate {
        match validate_mnemonic(&phrase_to_validate, &wordlist) {
            Ok(is_valid) => {
                if is_valid {
                    println!("Mnemonic is valid.");
                } else {
                    println!("Mnemonic is not valid.");
                }
            }
            Err(e) => eprintln!("Error validating mnemonic: {}", e),
        }
        return;
    }

    let mut phrases = Vec::new();
    for _ in 0..args.count {
        phrases.push(generate_mnemonic(&wordlist).unwrap());
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

fn validate_mnemonic(phrase: &str, wordlist: &[String]) -> Result<bool, &'static str> {
    let words: Vec<&str> = phrase.split(' ').collect();
    if words.len() != MNEMONIC_WORDS {
        return Ok(false);
    }

    let mut bits = Vec::new();
    for word in words {
        if let Some(index) = wordlist.iter().position(|w| w == word) {
            for i in 0..11 {
                bits.push((index >> (10 - i)) & 1 == 1);
            }
        } else {
            return Err("Word not found in wordlist");
        }
    }

    let entropy_bits = &bits[0..ENTROPY_BITS];
    let checksum_bits = &bits[ENTROPY_BITS..];

    let mut entropy = vec![0u8; ENTROPY_BITS / 8];
    for i in 0..ENTROPY_BITS {
        if entropy_bits[i] {
            entropy[i / 8] |= 1 << (7 - (i % 8));
        }
    }

    let expected_checksum = calculate_checksum(&entropy);
    Ok(checksum_bits == expected_checksum)
}

fn save_phrases(phrases: &[String], path: &str) -> io::Result<()> {
    let mut file = File::create(path)?;
    for phrase in phrases {
        writeln!(file, "{}", phrase)?;
    }
    Ok(())
}

fn read_wordlist<P: AsRef<Path>>(path: P) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

fn generate_mnemonic(wordlist: &[String]) -> Result<String, &'static str> {
    let mut entropy = vec![0u8; ENTROPY_BITS / 8];
    OsRng.fill_bytes(&mut entropy);

    let checksum = calculate_checksum(&entropy);
    let combined = combine_entropy_checksum(&entropy, checksum);

    let mut words = Vec::new();
    for i in 0..MNEMONIC_WORDS {
        let start = i * 11;
        let end = start + 11;
        let bits = &combined[start..end];
        let index = bits_to_usize(bits);
        words.push(wordlist[index].clone());
    }

    Ok(words.join(" "))
}

fn calculate_checksum(entropy: &[u8]) -> Vec<bool> {
    let mut hasher = Sha256::new();
    hasher.update(entropy);
    let hash = hasher.finalize();

    let mut checksum_bits = Vec::new();
    for i in 0..CHECKSUM_BITS {
        checksum_bits.push((hash[i / 8] >> (7 - (i % 8))) & 1 == 1);
    }
    checksum_bits
}

fn combine_entropy_checksum(entropy: &[u8], checksum: Vec<bool>) -> Vec<bool> {
    let mut entropy_bits = Vec::new();
    for byte in entropy {
        for i in 0..8 {
            entropy_bits.push((byte >> (7 - i)) & 1 == 1);
        }
    }
    entropy_bits.extend(checksum);
    entropy_bits
}

fn bits_to_usize(bits: &[bool]) -> usize {
    let mut num = 0;
    for (i, bit) in bits.iter().enumerate() {
        if *bit {
            num |= 1 << (bits.len() - 1 - i);
        }
    }
    num
}

use pyo3::prelude::*;
use rand::{RngCore, rngs::OsRng};
use sha2::{Sha256, Digest};
use std::io::{self};
use std::fs::File;
use std::io::Write;

#[pymodule]
fn bip39_generator_lib(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(generate_mnemonic_py, m)?)?;
    m.add_function(wrap_pyfunction!(validate_mnemonic_py, m)?)?;
    Ok(())
}

#[pyfunction]
fn generate_mnemonic_py() -> PyResult<String> {
    Ok(generate_mnemonic_phrase())
}

#[pyfunction]
fn validate_mnemonic_py(phrase: &str) -> PyResult<bool> {
    Ok(validate_mnemonic_phrase(phrase))
}

const ENTROPY_BITS: usize = 128;
const CHECKSUM_BITS: usize = ENTROPY_BITS / 32;
const MNEMONIC_WORDS: usize = (ENTROPY_BITS + CHECKSUM_BITS) / 11;

const WORDLIST: &str = include_str!("../bip39-english.txt");

pub fn generate_mnemonic_phrase() -> String {
    let wordlist: Vec<String> = WORDLIST.lines().map(String::from).collect();
    generate_mnemonic(&wordlist).unwrap()
}

pub fn validate_mnemonic_phrase(phrase: &str) -> bool {
    let wordlist: Vec<String> = WORDLIST.lines().map(String::from).collect();
    validate_mnemonic(phrase, &wordlist).unwrap_or(false)
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

pub fn save_phrases(phrases: &[String], path: &str) -> io::Result<()> {
    let mut file = File::create(path)?;
    for phrase in phrases {
        writeln!(file, "{}", phrase)?;
    }
    Ok(())
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

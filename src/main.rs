use anyhow::Result;
use crypto::{
    keys::{
        bip39::{mnemonic_to_seed, wordlist},
        slip10::Seed,
    },
    utils,
};

use std::io;
use std::thread;
use std::time::Instant;

mod address;

const BECH_32_CHARS: &str = "023456789acdefghjklmnpqrstuvwxyz";
const BECH_32_HRP: &str = "iota1q";
// last letter can actually be q, z, p, r
const SEPARATOR_AND_VERSION: &str = "1qp";

fn main() -> Result<()> {
    println!("Enter the chars you want to have at the beginning of the address and the amount of addresses you want to generate, in millions");
    println!("For example write \"Test 3\" to search for an address that has the chars from \"Test\" at the beginning and generate 3 million addresses to find such an address");
    println!(
        "Upper and lower case is ignored, output will look like this if an address was found:"
    );
    println!("Found address iota1q1qp00000j329n5gvc4y7lv5pfa60n0lg6ls0qflsk33hrtweq5j4h6hs2spp with mnemonic: young spread release team razor alert beef humor way august raise canyon despair spray danger please robust 
    artwork enroll sport crater document horn grid");

    let user_input = get_user_input();
    println!(
        "Searching for {} in {} million addresses...",
        user_input[0], user_input[1]
    );
    println!("Abort with Ctrl+C");

    let time_start = Instant::now();
    let threads = 16;
    let total_addresses = user_input[1].parse::<usize>().unwrap() * 1000000;
    let amount = total_addresses / threads;

    let beginning = user_input[0].to_ascii_uppercase();
    let arc_beginning = std::sync::Arc::new(beginning);

    let mut pool = vec![];
    for _ in 0..threads {
        let s = arc_beginning.clone();
        pool.push(thread::spawn(move || find_address(s, amount)));
    }
    for worker in pool {
        worker.join().unwrap()?;
    }
    println!("Done after {:.2?}", time_start.elapsed());
    Ok(())
}

fn find_address(beginning: std::sync::Arc<String>, amount: usize) -> Result<()> {
    // max address indexes to check for a single mnemonic
    let divider = 200;
    for _ in 0..amount / divider {
        let mut entropy = [0u8; 32];
        utils::rand::fill(&mut entropy)?;
        let mnemonic: String = wordlist::encode(&entropy, &crypto::keys::bip39::wordlist::ENGLISH)
            .map_err(|e| anyhow::anyhow!(format!("{:?}", e)))?;
        let mut mnemonic_seed = [0u8; 64];
        mnemonic_to_seed(&mnemonic, "", &mut mnemonic_seed);
        let seed = Seed::from_bytes(&mnemonic_seed);

        let range = BECH_32_HRP.len() + SEPARATOR_AND_VERSION.len()
            ..BECH_32_HRP.len() + SEPARATOR_AND_VERSION.len() + beginning.len();
        for i in 0..divider {
            let address = address::generate_address(&seed, 0, i.try_into().unwrap(), false)?;
            // println!("{}", &address.to_bech32(BECH_32_HRP)[range.clone()]);
            if *beginning == address.to_bech32(BECH_32_HRP)[range.clone()] {
                println!(
                    "Found address {} with mnemonic: {}",
                    address.to_bech32(BECH_32_HRP),
                    mnemonic
                );
            }
        }
    }
    Ok(())
}

pub fn get_user_input() -> Vec<String> {
    let mut input = String::new();
    let user_input = loop {
        let amount = 2;
        io::stdin().read_line(&mut input).unwrap();
        let words =
            input.trim_end_matches(|c| char::is_control(c) || char::is_whitespace(c) || c == '\n');
        let words: Vec<String> = words
            .split_ascii_whitespace()
            .map(|w| w.into())
            .collect::<Vec<_>>();
        if words.len() > amount {
            println!("Too many inputs, expected 2 (Word Amount)");
            input.clear();
        } else if words.len() < amount {
            println!("Too few inputs, expected 2 (first chars and amount)");
            input.clear();
        } else {
            //check if valid input
            //check first char
            // if !valid_first_chars
            //     .chars()
            //     .any(|c| c == words[0][0..1].chars().next().unwrap())
            // {
            //     println!("Invalid first char, only one of abJKLMNPQRSTUVWXYZ is allowed, please try again");
            //     input.clear();
            //     continue;
            // }

            //check for all other chars
            if words[0]
                .chars()
                .any(|c| !BECH_32_CHARS.chars().any(|d| d == c))
            {
                println!(
                    "Invalid char, only bech32 chars are allowed: {}",
                    BECH_32_CHARS
                );
                input.clear();
                continue;
            }
            match words[1].parse::<usize>() {
                Ok(_) => break words,
                Err(_) => {
                    println!(
                        "{} is not a valid number, only positive integers are allowed",
                        words[1]
                    );
                    input.clear();
                    continue;
                }
            }
        }
    };
    user_input
}

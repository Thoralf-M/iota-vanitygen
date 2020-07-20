use std::io;
use std::thread;
use std::time::Instant;

fn main() {
    println!("Enter the chars you want to have at the beginning of the address and the amount of addresses you want to generate, in millions");
    println!("For example write \"Test 3\" to search for an address that has the chars from \"Test\" at the beginning and generate 3 million addresses to find such an address");
    println!(
        "Upper and lower case is ignored, output will look like this if an address was found:"
    );
    println!("Found address TesTsEhMde7MmxvZxch9kWJgVtsrnL4ua4qoexAe51gi with seed FYrwHApFHSXMeuR6bovqomGgfrvtLmb7NtR62qfnRfSd");

    let user_input = get_user_input();
    println!(
        "Searching for {} in {} million addresses...",
        user_input[0], user_input[1]
    );
    println!("Abort with Ctrl+C");

    let time_start = Instant::now();
    let threads = 8;
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
        worker.join().unwrap();
    }
    println!("Done after {:.2?}", time_start.elapsed());
}

fn find_address(beginning: std::sync::Arc<String>, amount: usize) {
    for _ in 0..amount {
        let wallet = iota_ed25519_addresses::Wallet::new();
        if *beginning == wallet.address(0)[0..beginning.len()].to_ascii_uppercase() {
            println!(
                "Found address {} with seed {}",
                wallet.address(0),
                wallet.get_base58_seed()
            );
        }
    }
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
            let valid_first_chars = "abJKLMNPQRSTUVWXYZ";
            let invalid_base58_chars = "0OlI";
            let valid_base58_chars = "123456789ABCDEFGHJKLMNPQRSTUVWXYzabcdefghijkmnopqrstuvwxyz";
            //check first char
            if !valid_first_chars
                .chars()
                .any(|c| c == words[0][0..1].chars().next().unwrap())
            {
                println!("Invalid first char, only one of abJKLMNPQRSTUVWXYZ is allowed, please try again");
                input.clear();
                continue;
            }
            //check especially for "0OlI"
            if words[0]
                .chars()
                .any(|c| invalid_base58_chars.chars().any(|d| d == c))
            {
                println!("Invalid base58 char, chars of 0OlI are not allowed anywhere");
                input.clear();
                continue;
            }
            //check for all other chars
            if words[0]
                .chars()
                .any(|c| !valid_base58_chars.chars().any(|d| d == c))
            {
                println!("Invalid char, only base58 chars are allowed: 123456789ABCDEFGHJKLMNPQRSTUVWXYzabcdefghijkmnopqrstuvwxyz");
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

use std::io;
use rand::{thread_rng, Rng};

fn main() {
    let with_digits = with_digits();
    let with_letters = with_letters();
    let with_symbols = with_symbols();

    let alphabet: String = get_alphabet(with_digits, with_letters, with_symbols);
    let alphabet_length = alphabet.len();

    println!("Enter the number of passwords:");
    let mut quantity = String::new();

    io::stdin()
        .read_line(&mut quantity)
        .expect("Error");

    let quantity: usize = quantity
        .trim().parse()
        .expect("Error");

    println!("Enter password length:");
    let mut length = String::new();

    io::stdin()
        .read_line(&mut length)
        .expect("Error");

    let length: usize = length
        .trim().parse()
        .expect("Error");

    println!();

    for i in 1..=quantity {
        let mut password = String::new();

        for _ in 0..length {
            let index = thread_rng().gen_range(0..alphabet_length);

            password.push(alphabet.as_bytes()[index] as char);
        }

        println!("{}. {}", i, password);
    }

    let _ = io::stdin().read_line(&mut String::with_capacity(0));
}

fn with_digits() -> bool {
    println!("With digits? (y/n)");

    let mut with_digits = String::new();

    io::stdin()
        .read_line(&mut with_digits)
        .expect("Error");

    with_digits.trim().to_lowercase() == "y"
}

fn with_letters() -> bool {
    println!("With letters? (y/n)");

    let mut with_letters = String::new();

    io::stdin()
        .read_line(&mut with_letters)
        .expect("Error");

    with_letters.trim().to_lowercase() == "y"
}

fn with_symbols() -> bool {
    println!("With symbols? (y/n)");

    let mut with_symbols = String::new();

    io::stdin()
        .read_line(&mut with_symbols)
        .expect("Error");

    with_symbols.trim().to_lowercase() == "y"
}

fn get_alphabet(with_digits: bool, with_letters: bool, with_symbols: bool) -> String {
    let mut alphabet = String::new();

    if with_digits {
        for digit in '0'..='9' {
            alphabet.push(digit);
        }
    }

    if with_letters {
        for letter in 'a'..='z' {
            alphabet.push(letter);
        }

        for letter in 'A'..='Z' {
            alphabet.push(letter);
        }
    }

    if with_symbols {
        for symbol in '!'..='/' {
            alphabet.push(symbol);
        }
    }

    return alphabet;
}

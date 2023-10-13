use std::io;
use rand::{thread_rng, Rng};

fn main() {
    let with_digits = read_property("With digits? (y/n)");
    let with_letters = read_property("With letters? (y/n)");
    let with_symbols = read_property("With symbols? (y/n)");

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

fn read_property(message: &str) -> bool {
    println!("{}", message);

    let mut property = String::new();

    io::stdin()
        .read_line(&mut property)
        .expect("Error");

    property.trim().to_lowercase() == "y"
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

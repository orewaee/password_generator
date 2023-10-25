use std::io;
use rand::{thread_rng, Rng};

fn main() {
    let custom_alphabet = read_property("Custom alphabet? (y/n)");

    let alphabet: String;

    if custom_alphabet {
        println!("Input alphabet:");

        let mut value = String::new();

        io::stdin()
            .read_line(&mut value)
            .expect("Error");

        alphabet = value.trim().to_owned();
    } else {
        let with_digits = read_property("With digits? (y/n)");
        let with_letters = read_property("With letters? (y/n)");
        let with_symbols = read_property("With symbols? (y/n)");

        alphabet = get_alphabet(with_digits, with_letters, with_symbols);
    }

    let alphabet_length = alphabet.len();

    let quantity = read_number("Enter the number of passwords:");
    let length = read_number("Enter password length:");

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

fn read_number(message: &str) -> usize {
    println!("{}", message);

    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Error");

    let number: usize = number
        .trim().parse()
        .expect("Error");

    number
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

    alphabet
}

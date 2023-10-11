use std::io;
use rand::{thread_rng, Rng};



fn main() {
    let alphabet: String = get_alphabet();
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

fn get_alphabet() -> String {
    let mut alphabet = String::new();

    for c in '0'..='9' {
        alphabet.push(c);
    }

    for c in 'A'..='Z' {
        alphabet.push(c);
    }

    for c in 'a'..='z' {
        alphabet.push(c);
    }

    for c in '!'..='/' {
        alphabet.push(c);
    }

    return alphabet;
}

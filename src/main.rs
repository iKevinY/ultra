extern crate ultra;

use std::io::{stdin, stdout, Write};
use ultra::enigma::Enigma;

fn main() {
    let mut msg = String::new();
    print!("Message to encrypt: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut msg).unwrap();
    let msg = msg.trim_right();

    if !msg.is_empty() {
        let mut enigma = Enigma::new("123", "AAA", "AAA", 'B', "");
        println!("Ciphertext: {}", enigma.encrypt(msg))
    }
}

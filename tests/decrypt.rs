extern crate ultra;

mod texts;

use texts::TWO_CITIES;
use ultra::{decrypt, Enigma};

// Removes all non-alphabetic characters from a string and uppercases it.
macro_rules! preprocess {
    ($string:tt) => {
        $string.to_uppercase().chars()
            .filter(|&c| c.is_alphabetic())
            .collect::<String>()
    }
}

#[test]
fn two_cities_decryption() {
    let two_cities = preprocess!(TWO_CITIES);
    let mut enigma = Enigma::random_from_u64_seed(4);
    let ciphertext = enigma.encrypt(&two_cities);
    let (plaintext, _) = decrypt(&ciphertext);
    assert_eq!(plaintext, two_cities);
}

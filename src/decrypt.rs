use ordered_float::OrderedFloat;
use rayon::prelude::*;

use super::CharIndex;
use super::enigma::Enigma;

use fitness::{Quadgram, FitnessFn};

lazy_static! {
    static ref ALPHAS: Vec<String> = {
        // Construct the vector ["AAA", "AAB", ..., "ZZY", "ZZZ"]
        iproduct!(b'A'..b'[', b'A'..b'[', b'A'..b'[')
            .map(|(a, b, c)| String::from_utf8(vec![a, b, c]).unwrap())
            .collect()
    };

    static ref ROTORS: Vec<String> = {
        // Create all permutations of rotors 1 through 5
        iproduct!(b'1'..b'6', b'1'..b'6', b'1'..b'6')
            .filter(|&(r1, r2, r3)| (r1 != r2) && (r1 != r3) && (r2 != r3))
            .map(|(r1, r2, r3)| String::from_utf8(vec![r1, r2, r3]).unwrap())
            .collect()
    };
}


/// Decrypts the given string by iterating through multiple possible Enigma
/// configurations, returning the tuple `(plaintext, rotor, key, ring)`
/// corresponding to the most probable plaintext.
pub fn decrypt(msg: &str) -> (String, String, String, String) {
    // Try all rotor/key permutations (60*26^3 == 1,054,560 decryptions)
    let (rotor, key) = iproduct!(ROTORS.iter(), ALPHAS.iter())
        .collect::<Vec<_>>()
        .into_par_iter()
        .max_by_key(|&(rotor, key)| {
            let mut enigma = Enigma::new(rotor, key, "AAA", 'B', "");
            let plaintext = enigma.encrypt(msg);
            OrderedFloat(Quadgram::score(&plaintext))
        }).unwrap();

    // Keep the best rotor configuration found previously, and use the same
    // key setting for the first (slow) rotor. The ring setting for the first
    // rotor doesn't matter, so fix it to 'A', and try all key/ring settings
    // for the remaining two rotors (26^4 == 456,976 decryptions)
    let key_offset = key.chars().nth(0).unwrap().index() * 676;

    let (_, msg, key, ring) = iproduct!(key_offset..(key_offset + 676), 0..676)
        .collect::<Vec<_>>()
        .par_iter()
        .map(|&(key_index, ring_index)| {
            let key = &ALPHAS[key_index];
            let ring = &ALPHAS[ring_index];

            let mut enigma = Enigma::new(rotor, key, ring, 'B', "");
            let plaintext = enigma.encrypt(msg);
            let score = Quadgram::score(&plaintext);
            (OrderedFloat(score), plaintext, key, ring)
        }).max().unwrap();

    (msg, rotor.clone(), key.clone(), ring.clone())
}

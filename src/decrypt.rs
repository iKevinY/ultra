use itertools::Itertools;
use ordered_float::OrderedFloat;
use rayon::prelude::*;

use super::CharIndex;
use super::enigma::Enigma;

use constants::MAX_PLUGS;
use fitness::{IoC, Bigram, Quadgram, FitnessFn};

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
/// configurations, returning the tuple `(plaintext, Enigma)` corresponding to
/// the most probable plaintext.
///
/// Original decryption algorithm, uses quadgram frequency throughout.
pub fn decrypt(msg: &str) -> (String, Enigma) {
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

    (msg, Enigma::new(rotor, key, ring, 'B', ""))
}

/// Decrypts the given string by iterating through multiple possible Enigma
/// configurations, returning `(plaintext, Enigma)` corresponding to the most
/// probable plaintext.
///
/// Uses index of coincidence to pick rotors, bigram frequency to pick the
/// key and ring settings, and quadgram frequency for plugboard guesses.
pub fn plugboard_decrypt(msg: &str) -> (String, Enigma) {
    let (rotor, key) = iproduct!(ROTORS.iter(), ALPHAS.iter())
    .collect::<Vec<_>>()
    .into_par_iter()
    .max_by_key(|&(rotor, key)| {
        let mut enigma = Enigma::new(rotor, key, "AAA", 'B', "");
        OrderedFloat(IoC::score(&enigma.encrypt(msg)))
    }).unwrap();

    // Keep the best rotor configuration found previously, and use the same
    // key setting for the first (slow) rotor. The ring setting for the first
    // rotor doesn't matter, so fix it to 'A', and try all key/ring settings
    // for the remaining two rotors (26^4 == 456,976 decryptions)
    let key_offset = key.chars().nth(0).unwrap().index() * 676;

    let (_, key, ring) = iproduct!(key_offset..(key_offset + 676), 0..676)
        .collect::<Vec<_>>()
        .par_iter()
        .map(|&(key_index, ring_index)| {
            let key = &ALPHAS[key_index];
            let ring = &ALPHAS[ring_index];

            let mut enigma = Enigma::new(rotor, key, ring, 'B', "");
            let score = Bigram::score(&enigma.encrypt(msg));
            (OrderedFloat(score), key, ring)
        }).max().unwrap();

    let mut plug_pool: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let mut best_plugboard = String::new();
    let mut best_score = Quadgram::score(&msg);
    let mut best_plaintext = String::new();

    for _ in 0..MAX_PLUGS {
        let pool = plug_pool.clone();
        let combos = pool.iter().combinations(2).collect::<Vec<_>>();
        let (score, best_guess, plugs, plugboard) = combos.par_iter()
            .map(|plugs| {
                let plugboard = format!("{} {}{}", best_plugboard, plugs[0], plugs[1]);
                let mut enigma = Enigma::new(rotor, key, ring, 'B', &plugboard.trim());
                let plaintext = enigma.encrypt(&msg);
                let score = Quadgram::score(&plaintext);
                (OrderedFloat(score), plaintext, plugs, plugboard)
            }).max().unwrap();
        if *score > best_score {
            best_score = *score;
            best_plaintext = best_guess.clone();
            best_plugboard = plugboard.trim().to_string();
            plug_pool.retain(|&c| c != *plugs[0] && c != *plugs[1]);
        } else {
            break;
        }
    }

    (best_plaintext, Enigma::new(rotor, key, ring, 'B', &best_plugboard))
}

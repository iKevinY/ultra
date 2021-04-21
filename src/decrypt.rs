use std::iter;

use itertools::Itertools;
use ordered_float::OrderedFloat;
use rayon::prelude::*;

use super::CharIndex;
use super::enigma::Enigma;

use constants::MAX_PLUGS;
use fitness::{IoC, Bigram, Quadgram, FitnessFn};

lazy_static! {
    // The vector of key/ring settings ["AAA", "AAB", ..., "ZZY", "ZZZ"].
    static ref ALPHAS: Vec<String> = {
        iproduct!('A'..='Z', 'A'..='Z', 'A'..='Z')
            .map(|(a, b, c)| [a, b, c].iter().collect())
            .collect()
    };

    // The vector of permutations ["123", "124", "125", ..., "542", "543"].
    static ref ROTORS: Vec<String> = {
        ('1'..='5').permutations(3)
            .map(|p| p.iter().collect())
            .collect()
    };
}


/// Decrypts the given string by iterating through multiple possible Enigma
/// configurations, returning `(plaintext, Enigma)` corresponding to the most
/// probable decryption.
///
/// The decryption algorithm works in three steps:
///
/// 1. Guesses the rotors and first key setting.
/// 2. Guesses the remaining key settings and ring settings.
/// 3. Incrementally adds the best plug until no improvement is made.
///
/// Assumes `msg` contains only uppercase ASCII characters.
pub fn decrypt(msg: &str) -> (String, Enigma) {
    let mut enigma;

    enigma = guess_rotor_and_first_key::<IoC>(msg);
    enigma = guess_key_and_ring::<Bigram>(msg, enigma);
    enigma = guess_plugboard::<Quadgram>(msg, enigma);

    (enigma.encrypt(msg), enigma)
}


/// Given a piece of ciphertext and a fitness function, tries all valid rotor
/// and key combinations, and returns an `Enigma` with rotor settings and
/// first key setting that best decrypt the ciphertext. Note that the other
/// key settings and ring settings will be meaningless at this point.
///
/// This method checks 60 * 26^3 == 1,054,560 settings in parallel.
fn guess_rotor_and_first_key<F: FitnessFn>(msg: &str) -> Enigma {
    let (rotor, key) = iproduct!(ROTORS.iter(), ALPHAS.iter())
        .collect::<Vec<_>>()
        .into_par_iter()
        .max_by_key(|&(rotor, key)| {
            let mut enigma = Enigma::new(rotor, key, "AAA", 'B', "");
            OrderedFloat(F::score(&enigma.encrypt(msg)))
        }).unwrap();

    Enigma::new(rotor, key, "AAA", 'B', "")
}

/// Given a piece of ciphertext, fitness function, and an `Enigma` instance
/// with the correct rotors and first key setting, tries all key and ring
/// settings for the mid and fast rotors, and returns an `Enigma` with the
/// best rotors, key, and ring settings assigned.
///
/// This method checks 26^4 == 456,976 settings in parallel.
fn guess_key_and_ring<F: FitnessFn>(msg: &str, enigma: Enigma) -> Enigma {
    let rotor = enigma.rotor_list();
    let first_key = enigma.key_settings().chars().next().unwrap();

    // Compute the key offset based on the first key setting, so that we only
    // iterate over the next 676 key settings (this implicitly fixes the slow
    // rotor's key setting at 'A', which is intentional because this setting
    // does not affect the overall decryption).
    let key_offset = first_key.index() * 676;

    let keys = &ALPHAS[key_offset..(key_offset + 676)];
    let rings = &ALPHAS[0..676];

    let (key, ring) = iproduct!(keys, rings)
        .collect::<Vec<_>>()
        .into_par_iter()
        .max_by_key(|&(key, ring)| {
            let mut enigma = Enigma::new(&rotor, key, ring, 'B', "");
            OrderedFloat(F::score(&enigma.encrypt(msg)))
        }).unwrap();

    Enigma::new(&rotor, key, ring, 'B', "")
}

/// Given a piece of ciphertext, fitness function, and an `Enigma` instance
/// with the correct rotors, key, and ring settings, attempts to add plugs
/// to the plugboard until adding one does not increase the overall fitness
/// of the decryption (or the number of plugs equals `MAX_PLUGS`). Returns the
/// resulting `Enigma`.
///
/// At most, this is MAX_PLUGS * 26^2 == 6,760 tests.
fn guess_plugboard<F: FitnessFn>(msg: &str, enigma: Enigma) -> Enigma {
    let rotor = enigma.rotor_list();
    let key = enigma.key_settings();
    let ring = enigma.ring_settings();

    let mut curr_plugboard = Vec::new();
    let mut plug_pool: Vec<char> = ('A'..='Z').collect();
    let mut best_score = F::score(&msg);

    for _ in 0..MAX_PLUGS {
        let plugs: Vec<String> = plug_pool
            .iter()
            .combinations(2)
            .map(|p| p.into_iter().collect())
            .collect();

        let (score, plug) = plugs.par_iter()
            .map(|plug| {
                let plugboard = curr_plugboard
                    .iter()
                    .chain(iter::once(plug))
                    .join(" ");

                let mut enigma = Enigma::new(&rotor, &key, &ring, 'B', &plugboard);
                (OrderedFloat(F::score(&enigma.encrypt(&msg))), plug)
            }).max().unwrap();

        if *score > best_score {
            best_score = *score;
            curr_plugboard.push(plug.to_string());
            plug_pool.retain(|&c| !plug.chars().any(|p| p == c));
        } else {
            break;
        }
    }

    Enigma::new(&rotor, &key, &ring, 'B', &curr_plugboard.iter().join(" "))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn static_vars() {
        assert_eq!(ALPHAS.len(), 26 * 26 * 26);
        assert_eq!(ROTORS.len(), 120 / 2);  // n! / (n - k)!
    }
}


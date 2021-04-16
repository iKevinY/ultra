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

type ScoreFn = fn(&str) -> f64;

/// Decrypts the given string by iterating through multiple possible Enigma
/// configurations, returning `(plaintext, Enigma)` corresponding to the most
/// probable decryption.
///
/// Assumes `msg` contains only uppercase ASCII characters.
pub fn decrypt(msg: &str) -> (String, Enigma) {
    let (rotor, first_key) = guess_rotor_and_first_key(msg, IoC::score);
    let (key, ring) = guess_key_and_ring(msg, Bigram::score, rotor.clone(), first_key);
    let mut enigma = guess_plugboard(msg, Quadgram::score, rotor, key, ring);

    (enigma.encrypt(msg), enigma)
}

/// Given a piece of ciphertext and a fitness function, tries all valid rotor
/// and key combinations, and returns the rotor settings and first key that
/// best decrypt the ciphertext.
fn guess_rotor_and_first_key(msg: &str, score_fn: ScoreFn) -> (String, char) {
    let (rotor, key) = iproduct!(ROTORS.iter(), ALPHAS.iter())
        .collect::<Vec<_>>()
        .into_par_iter()
        .max_by_key(|&(rotor, key)| {
            let mut enigma = Enigma::new(rotor, key, "AAA", 'B', "");
            OrderedFloat(score_fn(&enigma.encrypt(msg)))
        }).unwrap();

    (rotor.clone(), key.chars().nth(0).unwrap())
}

/// Given the a set of rotors and the first key setting, tries all 26^4 ==
/// 456,976 key and ring settings for the mid and fast rotors, and returns
/// the key and ring settings corresponding to the best decryption.
fn guess_key_and_ring(msg: &str, score_fn: ScoreFn, rotor: String, first_key: char)
    -> (String, String) {
    // Compute the key offset based on the first key setting, so that we only
    // iterate over the next 676 key settings (this implicitly fixes the slow
    // rotor's key setting at 'A', which is intentional because this setting
    // does not affect the overall decryption).
    let key_offset = first_key.index() * 676;

    let (_, key, ring) = iproduct!(key_offset..(key_offset + 676), 0..676)
        .collect::<Vec<_>>()
        .par_iter()
        .map(|&(key_index, ring_index)| {
            let key = &ALPHAS[key_index];
            let ring = &ALPHAS[ring_index];

            let mut enigma = Enigma::new(&rotor, key, ring, 'B', "");
            (OrderedFloat(score_fn(&enigma.encrypt(msg))), key, ring)
        }).max().unwrap();

    (key.clone(), ring.clone())
}

/// Given rotor, key, and ring settings, repeatedly adds plugs until adding
/// a plug does not increase the overall fitness of the decryption. Returns
/// the resulting `Enigma`.
fn guess_plugboard(msg: &str, score_fn: ScoreFn, rotor: String, key: String,
        ring: String) -> Enigma {
    let mut plug_pool: Vec<char> = ('A'..='Z').collect();

    let mut best_plugboard = "".to_string();
    let mut best_score = score_fn(&msg);

    for _ in 0..MAX_PLUGS {
        let pool = plug_pool.clone();
        let combos = pool.iter().combinations(2).collect::<Vec<_>>();
        let (score, plugs, plugboard) = combos.par_iter()
            .map(|plugs| {
                let plugboard = format!("{} {}{}", best_plugboard, plugs[0], plugs[1]);
                let mut enigma = Enigma::new(&rotor, &key, &ring, 'B', &plugboard.trim());
                (OrderedFloat(score_fn(&enigma.encrypt(&msg))), plugs, plugboard)
            }).max().unwrap();

        if *score > best_score {
            best_score = *score;
            best_plugboard = plugboard.trim().to_string();
            plug_pool.retain(|&c| c != *plugs[0] && c != *plugs[1]);
        } else {
            break;
        }
    }

    Enigma::new(&rotor, &key, &ring, 'B', &best_plugboard)
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


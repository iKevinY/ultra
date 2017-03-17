use ordered_float::OrderedFloat;
use rayon::prelude::*;

use super::CharIndex;
use super::enigma::Enigma;

lazy_static! {
    static ref QGRAMS: Vec<f64> = {
        let f = include_str!("data/quadgrams.txt");
        let mut qgrams = vec![0.0; 456_976];  // 26^4 = 456,976

        for line in f.lines() {
            let line: Vec<_> = line.split(' ').collect();
            let qgram: &str = line[0];
            let count: f64 = line[1].parse().unwrap();

            let index = qgram.chars().fold(0, |acc, c| 26 * acc + c.index());
            qgrams[index] = count.ln();
        }

        qgrams
    };

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
            OrderedFloat(qgram_score(&plaintext))
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
            let score = qgram_score(&plaintext);
            (OrderedFloat(score), plaintext, key, ring)
        }).max().unwrap();

    (msg, rotor.clone(), key.clone(), ring.clone())
}


/// Strips all non-alphabetic characters from the given message string and
/// returns the sum of the log-probabilities of each quadgram substring.
pub fn qgram_score(msg: &str) -> f64 {
    let char_indices: Vec<usize> = msg.chars()
        .filter(|&c| c.is_alphabetic())
        .map(|c| c.index())
        .collect();

    if char_indices.len() < 4 {
        panic!("Message must contain 4 or more alphabetic characters.");
    }

    char_indices.windows(4)
        .map(|w| w.iter().fold(0, |acc, x| 26 * acc + x))
        .map(|i| QGRAMS[i])
        .sum()
}


#[cfg(test)]
mod tests {
    use super::{QGRAMS, qgram_score};

    macro_rules! assert_approx_eq {
        ($a:expr, $b:expr) => {
            let (a, b) = (&$a, &$b);
            assert!((*a - *b).abs() < 1.0e-6, "{} is not approximately equal to {}", *a, *b);
        }
    }

    #[test]
    fn qgram_estimates() {
        assert_approx_eq!(QGRAMS[0], 8.81060879);
        assert_approx_eq!(qgram_score("THE QUICK BROWN FOX"), 149.80102862);
    }

    #[test]
    fn sensible_qgram_scores() {
        assert!(qgram_score("AN ENGLISH PHRASE") > qgram_score("ESARHP HSILGNE NA"));
    }

    #[test]
    #[should_panic]
    fn invalid_qgram_check() {
        qgram_score("ABC");
    }
}

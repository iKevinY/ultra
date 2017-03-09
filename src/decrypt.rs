use std::iter::FromIterator;

use super::{CharIndex, ToChar};
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
}


/// Decrypts the given string by iterating through multiple possible Enigma
/// configurations, returning the tuple `(plaintext, key, ring, rotor)`
/// corresponding to the most probable plaintext.
pub fn decrypt(msg: &str) -> (String, String, String, String) {
    let mut best_score = 0.0;
    let mut best_msg = msg.to_owned();
    let mut best_key = "AAA".to_owned();
    let mut best_ring = "AAA".to_owned();
    let mut best_rotor = "123".to_owned();

    // Rotor and key settings (60*26^3 == 1,054,560 decryptions)
    let r = "12345".chars();

    for (slow, mid, fast) in iproduct!(r.clone(), r.clone(), r.clone()) {
        // Skip rotor combinations that contain duplicates
        if (slow == mid) || (slow == fast) || (mid == fast) {
            continue;
        }

        let rotor = String::from_iter(vec![slow, mid, fast]);

        for (a, b, c) in iproduct!(0..26, 0..26, 0..26) {
            let key = String::from_iter(vec![a.to_char(), b.to_char(), c.to_char()]);

            let mut enigma = Enigma::new(&rotor, &key, "AAA", 'B', "");
            let plaintext = enigma.encrypt(msg);
            let score = qgram_score(&plaintext);

            if score > best_score {
                best_score = score;
                best_msg = plaintext;
                best_key = key;
                best_rotor = rotor.clone();
            }
        }
    }

    let first_key = best_key.chars().nth(0).unwrap();

    // Key and ring settings (26^4 == 456,976 decryptions)
    for (key2, key3, ring2, ring3) in iproduct!(0..26, 0..26, 0..26, 0..26) {
        let key = String::from_iter(vec![first_key, key2.to_char(), key3.to_char()]);
        let ring = String::from_iter(vec!['A', ring2.to_char(), ring3.to_char()]);

        let mut enigma = Enigma::new(&best_rotor, &key, &ring, 'B', "");
        let plaintext = enigma.encrypt(msg);
        let score = qgram_score(&plaintext);

        if score > best_score {
            best_score = score;
            best_msg = plaintext;
            best_key = key;
            best_ring = ring;
        }
    }

    (best_msg, best_key, best_ring, best_rotor)
}


/// Strips all non-alphabetic characters from the given message string and
/// returns the sum of the log-probabilities of each quadgram substring.
pub fn qgram_score(msg: &str) -> f64 {
    let char_indices: Vec<usize> = msg.chars()
        .filter(|&c| c.is_alphabetic())
        .map(|c| c.index())
        .collect();

    if char_indices.len() < 4 {
        panic!("Input string must be longer than 4 characters.");
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

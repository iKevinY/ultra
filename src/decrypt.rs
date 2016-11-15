use std::f64::{EPSILON, MIN};
use std::iter::FromIterator;

use super::{CharIndex, ToChar};
use super::enigma::Enigma;

lazy_static! {
    static ref QGRAMS: Vec<f64> = {
        let f = include_str!("data/quadgrams.txt");

        let mut total: usize = 0;
        let mut qgrams = Vec::new();

        for line in f.lines() {
            let mut iter = line.split(' ');
            let qgram = iter.next().unwrap();
            let count = iter.next().unwrap().parse().unwrap();

            qgrams.push((qgram, count));
            total += count;
        }

        let mut v = vec![EPSILON.ln(); 456_976];

        for (qgram, n) in qgrams {
            let frequency = (n as f64) / (total as f64);
            let index = qgram.chars().fold(0, |acc, c| 26 * acc + c.index());
            v[index] = frequency.ln();
        }

        v
    };
}


/// Decrypts the given string by iterating through multiple possible Enigma
/// configurations, returning the tuple `(plaintext, key, ring, rotor)`
/// corresponding to the most probable plaintext.
pub fn decrypt(msg: &str) -> (String, String, String, String) {
    let mut best_score = MIN;
    let mut best_msg = msg.to_owned();
    let mut best_key = "AAA".to_owned();
    let mut best_ring = "AAA".to_owned();
    let mut best_rotor = "123".to_owned();

    // Rotor and key settings (60*26^3 == 1,054,560 decryptions)
    let r = vec!['1', '2', '3', '4', '5'].into_iter();

    for (slow, mid, fast) in iproduct!(r.clone(), r.clone(), r.clone()) {
        // Skip rotor combinations that contain duplicates
        if (slow == mid) || (slow == fast) || (mid == fast) {
            continue;
        }

        for (a, b, c) in iproduct!(0..26, 0..26, 0..26) {
            let rotor = String::from_iter(vec![slow, mid, fast]);
            let key = String::from_iter(vec![a.to_char(), b.to_char(), c.to_char()]);

            let mut enigma = Enigma::new(&rotor, &key, "AAA", 'B', "");
            let plaintext = enigma.encrypt(msg);
            let score = qgram_score(&plaintext);

            if score > best_score {
                best_score = score;
                best_msg = plaintext;
                best_key = key;
                best_rotor = rotor;
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
    let msg: Vec<usize> = msg.chars()
        .filter(|&c| c.is_alphabetic())
        .map(|c| c.index())
        .collect();

    if msg.len() < 4 {
        panic!("Input string must be longer than 4 characters.");
    }

    let mut sum = 0.0;

    for i in 0..(msg.len() - 3) {
        let a = msg[i];
        let b = msg[i + 1];
        let c = msg[i + 2];
        let d = msg[i + 3];

        let index = (((a * 26) + b) * 26 + c) * 26 + d;
        sum += QGRAMS[index];
    }

    sum
}


#[cfg(test)]
mod tests {
    use super::qgram_score;

    #[test]
    fn qgram_estimate() {
        // Score should be approximately -138.3319940227875
        assert!((qgram_score("THE QUICK BROWN FOX") + 138.3).abs() < 0.1);
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

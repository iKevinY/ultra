use std::collections::HashMap;
use std::f64::EPSILON;


lazy_static! {
    static ref QGRAMS: HashMap<&'static str, f64> = {
        let mut map = HashMap::new();
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

        for (qgram, n) in qgrams {
            let frequency = (n as f64) / (total as f64);
            map.insert(qgram, frequency.ln());
        }

        map
    };
}


/// Strips all non-alphabetic characters from the given message string and
/// returns the sum of the log-probabilities of each quadgram substring.
pub fn qgram_score(msg: &str) -> f64 {
    let msg: String = msg.chars().filter(|&c| c.is_alphabetic()).collect();

    if msg.len() < 4 {
        panic!("Input string must be longer than 4 characters.");
    }

    (0..(msg.len() - 3))
        .map(|i| *QGRAMS.get(&msg[i..i+4]).unwrap_or(&EPSILON.ln()))
        .sum()
}


#[cfg(test)]
mod tests {
    use super::qgram_score;

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

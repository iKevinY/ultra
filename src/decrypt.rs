use std::f64::EPSILON;
use super::CharIndex;


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


/// Strips all non-alphabetic characters from the given message string and
/// returns the sum of the log-probabilities of each quadgram substring.
pub fn qgram_score(msg: &str) -> f64 {
    let msg: Vec<usize> = msg
        .chars()
        .filter(|&c| c.is_alphabetic())
        .map(|c| c.index())
        .collect();

    if msg.len() < 4 {
        panic!("Input string must be longer than 4 characters.");
    }

    let mut sum = 0.0;

    for i in 0..(msg.len() - 3) {
        let a = msg[i];
        let b = msg[i+1];
        let c = msg[i+2];
        let d = msg[i+3];

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

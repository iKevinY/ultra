use super::CharIndex;

lazy_static! {
    static ref BIGRAMS: Vec<f64> = {
        let f = include_str!("data/bigrams.txt");
        let mut bigrams = vec![0.0; 676];  // 26^2 = 676

        for line in f.lines() {
            let line: Vec<_> = line.split(' ').collect();
            let bigram: &str = line[0];
            let count: f64 = line[1].parse().unwrap();

            let index = bigram.chars().fold(0, |acc, c| 26 * acc + c.index());
            bigrams[index] = count.ln();
        }

        bigrams
    };

    static ref TRIGRAMS: Vec<f64> = {
        let f = include_str!("data/trigrams.txt");
        let mut trigrams = vec![0.0; 17_576];  // 26^3 = 17,576

        for line in f.lines() {
            let line: Vec<_> = line.split(' ').collect();
            let trigram: &str = line[0];
            let count: f64 = line[1].parse().unwrap();

            let index = trigram.chars().fold(0, |acc, c| 26 * acc + c.index());
            trigrams[index] = count.ln();
        }

        trigrams
    };

    static ref QUADGRAMS: Vec<f64> = {
        let f = include_str!("data/quadgrams.txt");
        let mut quadgrams = vec![0.0; 456_976];  // 26^4 = 456,976

        for line in f.lines() {
            let line: Vec<_> = line.split(' ').collect();
            let quadgram: &str = line[0];
            let count: f64 = line[1].parse().unwrap();

            let index = quadgram.chars().fold(0, |acc, c| 26 * acc + c.index());
            quadgrams[index] = count.ln();
        }

        quadgrams
    };
}


/// For a given `n` and `ngram` vector, returns the sum of log-probabilities
/// for each n-gram substring.
fn ngram_score(n: usize, ngrams: &Vec<f64>, msg: &str) -> f64 {
    let char_indices: Vec<usize> = msg.chars()
        .filter(|&c| c.is_alphabetic())
        .map(|c| c.index())
        .collect();

    if char_indices.len() < n {
        panic!("Message has fewer than n alphabetic characters.");
    }

    char_indices.windows(n)
        .map(|w| w.iter().fold(0, |acc, x| 26 * acc + x))
        .map(|i| ngrams[i])
        .sum()
}


pub trait FitnessFn {
    /// Returns the fitness score for a given message after stripping away
    /// non-alphabetic characters.
    fn score(msg: &str) -> f64;
}

/// Bigram probability fitness function for English.
pub struct Bigram;
impl FitnessFn for Bigram {
    fn score(msg: &str) -> f64 {
        ngram_score(2, &BIGRAMS, msg)
    }
}

/// Trigram probability fitness function for English.
pub struct Trigram;
impl FitnessFn for Trigram {
    fn score(msg: &str) -> f64 {
        ngram_score(3, &TRIGRAMS, msg)
    }
}

/// Quadgram probability fitness function for English.
pub struct Quadgram;
impl FitnessFn for Quadgram {
    fn score(msg: &str) -> f64 {
        ngram_score(4, &QUADGRAMS, msg)
    }
}

/// Returns the index of coincidence for the given message.
pub struct IoC;
impl FitnessFn for IoC {
    fn score(msg: &str) -> f64 {
        let char_indices: Vec<usize> = msg.chars()
            .filter(|&c| c.is_alphabetic())
            .map(|c| c.index())
            .collect();

        let mut buckets = [0; 26];

        for c in &char_indices {
            buckets[*c] += 1;
        }

        let tot: isize = buckets
            .iter()
            .map(|n| n * (n - 1))
            .sum();

        let n = char_indices.len();
        return tot as f64 / (n * (n - 1) / 26) as f64;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_approx_eq {
        ($a:expr, $b:expr) => {
            let (a, b) = (&$a, &$b);
            assert!((*a - *b).abs() < 1.0e-6, "{} is not approximately equal to {}", *a, *b);
        }
    }

    #[test]
    fn quadgram_estimates() {
        assert_approx_eq!(QUADGRAMS[0], 8.81060879);
        assert_approx_eq!(Quadgram::score("THE QUICK BROWN FOX"), 149.80102862);
    }

    #[test]
    fn sensible_ngram_scores() {
        assert!(Bigram::score("AN ENGLISH PHRASE") > Bigram::score("ESARHP HSILGNE NA"));
        assert!(Trigram::score("AN ENGLISH PHRASE") > Trigram::score("ESARHP HSILGNE NA"));
        assert!(Quadgram::score("AN ENGLISH PHRASE") > Quadgram::score("ESARHP HSILGNE NA"));
    }

    #[test]
    #[should_panic]
    fn invalid_qgram_check() {
        Quadgram::score("ABC");
    }

    #[test]
    fn sensible_ioc_scores() {
        assert_approx_eq!(IoC::score("THE INDEX OF COINCIDENCE PROVIDES A MEASURE OF HOW LIKELY IT IS TO DRAW TWO MATCHING LETTERS BY RANDOMLY SELECTING TWO LETTERS FROM A GIVEN TEXT"), 1.55925925);

        assert_approx_eq!(IoC::score(&"ABCDEFGHIJKLMNOPQRSTUVWXYZ".repeat(100)), 0.99038091);
    }
}

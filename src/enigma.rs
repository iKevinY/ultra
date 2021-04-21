use std::fmt;

use itertools::Itertools;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use rand::seq::SliceRandom;

use constants::MAX_PLUGS;
use plugboard::Plugboard;
use reflector::Reflector;
use rotor::Rotor;

/// Represents an Enigma machine with rotor, key, and ring settings.
#[derive(Clone, Debug)]
pub struct Enigma {
    slow: Rotor,
    mid: Rotor,
    fast: Rotor,
    reflector: Reflector,
    plugboard: Plugboard,
}

impl Enigma {
    /// Creates a new `Enigma`, where `rotors` is a string of three digits
    /// ranging from 1-8 (corresponding to rotors I through VIII of the real
    /// Enigma machine), `keys` and `rings` are three character strings
    /// containing the key and ring settings, `reflector` is one of `'A'`,
    /// `'B'`, or `'C'`, and `plugboard` is a string of whitespace-delimited
    /// pairs of characters.
    ///
    /// # Examples
    ///
    /// `Enigma` with rotors I, II, and III, key setting `ABC`, ring setting
    /// `DEF`, reflector B, and a plugboard connection between 'P' and 'Y'.
    ///
    /// ```
    /// use ultra::Enigma;
    ///
    /// let mut enigma = Enigma::new("123", "ABC", "DEF", 'B', "PY");
    /// ```
    pub fn new(rotors: &str, keys: &str, rings: &str, reflector: char, plugboard: &str) -> Enigma {
        let rotors: Vec<usize> = rotors.chars()
            .filter_map(|c| c.to_digit(10))
            .map(|n| n as usize)
            .collect();

        if rotors.len() != 3 {
            panic!("Exactly 3 rotors must be given.");
        }

        let keys: Vec<char> = keys.chars().collect();
        let rings: Vec<char> = rings.chars().collect();

        Enigma {
            slow: Rotor::from_enigma(rotors[0], keys[0], rings[0]),
            mid: Rotor::from_enigma(rotors[1], keys[1], rings[1]),
            fast: Rotor::from_enigma(rotors[2], keys[2], rings[2]),
            reflector: Reflector::from_enigma(reflector),
            plugboard: Plugboard::new(plugboard),
        }
    }

    /// Creates a new random `Enigma` with random settings based on
    /// thread-local RNG.
    ///
    /// # Examples
    ///
    /// ```
    /// use ultra::Enigma;
    ///
    /// let mut enigma_1 = Enigma::random();
    /// let mut enigma_2 = Enigma::random();
    /// assert!(enigma_1.encrypt("ENIGMA") != enigma_2.encrypt("ENIGMA"));
    /// ```
    pub fn random() -> Enigma {
        Enigma::random_from_rng(&mut rand::thread_rng())
    }

    /// Creates a new random `Enigma` from a given u64 seed.
    ///
    /// # Examples
    ///
    /// ```
    /// use ultra::Enigma;
    ///
    /// let mut enigma_1 = Enigma::random_from_u64_seed(42);
    /// let mut enigma_2 = Enigma::random_from_u64_seed(42);
    /// assert_eq!(enigma_1.encrypt("ENIGMA"), enigma_2.encrypt("ENIGMA"));
    /// ```
    pub fn random_from_u64_seed(seed: u64) -> Enigma {
        Enigma::random_from_rng(&mut StdRng::seed_from_u64(seed))
    }

    fn random_from_rng<R: Rng>(rng: &mut R) -> Enigma {
        let rotors: String = {
            let mut rotor_pool: Vec<char> = "12345".chars().collect();
            rotor_pool.shuffle(rng);
            rotor_pool[..3].iter().collect()
        };

        // Randomize key and ring settings for the rotors.
        let mut alpha: Vec<char> = ('A'..='Z').collect();

        alpha.shuffle(rng);
        let key: String = alpha[..3].iter().collect();

        alpha.shuffle(rng);
        let ring: String = alpha[..3].iter().collect();

        // Pick random plugs to fill plugboard with.
        alpha.shuffle(rng);
        let plugboard = alpha
            .chunks(2)
            .take(rng.gen_range(0..=MAX_PLUGS))
            .map(|chrs| chrs.iter().collect::<String>())
            .join(" ");

        Enigma::new(&rotors, &key, &ring, 'B', &plugboard)
    }


    /// Encrypts an entire message, advancing the rotors of the machine
    /// after each alphabetic character is encrypted.
    ///
    /// # Examples
    ///
    /// ```
    /// use ultra::Enigma;
    ///
    /// let mut enigma = Enigma::new("123", "ABC", "DEF", 'B', "PY");
    /// assert_eq!(enigma.encrypt("ENIGMA"), "HKAJWW");
    /// ```
    pub fn encrypt(&mut self, msg: &str) -> String {
        msg.chars().map(|c| self.encrypt_char(c)).collect()
    }

    /// Advances the rotors then returns the substitution of
    /// a character, if the input character was alphabetic.
    fn encrypt_char(&mut self, c: char) -> char {
        if !c.is_ascii() || !c.is_alphabetic() {
            return c;
        }

        self.advance();
        self.substitute(c.to_ascii_uppercase())
    }


    /// Returns the substitution of a character, which is determined by
    /// passing the character in sequence through the plugboard, the rotors
    /// from `fast` to `slow`, through the reflector, inverted through the
    /// rotors from `slow` to `fast`, and finally through the plugboard.
    fn substitute(&self, c: char) -> char {
        let mut c = self.plugboard.map(c);
        c = self.slow.substitute(self.mid.substitute(self.fast.substitute(c)));
        c = self.reflector.reflect(c);
        c = self.fast.invert(self.mid.invert(self.slow.invert(c)));
        self.plugboard.map(c)
    }

    /// Advances the `fast` rotor, and also advances the
    /// `mid` and `slow` rotors if appropriate.
    fn advance(&mut self) {
        // Check for double-rotation situation
        if self.mid.at_notch() {
            self.mid.advance();
            self.slow.advance();
        } else if self.fast.at_notch() {
            self.mid.advance();
        }

        // Finally, advance the fast rotor
        self.fast.advance();
    }

    /// Resets the `Enigma` to its initial state.
    ///
    /// # Examples
    ///
    /// ```
    /// use ultra::Enigma;
    ///
    /// let msg = "THIS IS A TEST";
    ///
    /// let mut enigma = Enigma::random();
    /// let ciphertext_1 = enigma.encrypt(msg);
    /// let ciphertext_2 = enigma.encrypt(msg);
    ///
    /// enigma.reset();
    ///
    /// assert_eq!(ciphertext_1, enigma.encrypt(msg));
    /// assert!(ciphertext_1 != ciphertext_2);
    /// ```
    pub fn reset(&mut self) {
        self.slow.reset();
        self.mid.reset();
        self.fast.reset();
    }


    /// Returns a string representing the `Enigma`'s rotor list.
    ///
    /// # Examples
    ///
    /// ```
    /// use ultra::Enigma;
    ///
    /// let enigma = Enigma::new("123", "ABC", "DEF", 'B', "PY");
    /// assert_eq!(enigma.rotor_list(), "123");
    /// ```
    pub fn rotor_list(&self) -> String {
        self.rotors().map(|r| r.to_string()).collect()
    }

    /// Returns a string representing the `Enigma`'s key settings.
    ///
    /// # Examples
    ///
    /// ```
    /// use ultra::Enigma;
    ///
    /// let enigma = Enigma::new("123", "ABC", "DEF", 'B', "PY");
    /// assert_eq!(enigma.key_settings(), "ABC");
    /// ```
    pub fn key_settings(&self) -> String {
        self.rotors()
            .map(|r| ((r.key_setting as u8) + b'A') as char)
            .collect()
    }

    /// Returns a string representing the `Enigma`'s ring settings.
    ///
    /// # Examples
    ///
    /// ```
    /// use ultra::Enigma;
    ///
    /// let enigma = Enigma::new("123", "ABC", "DEF", 'B', "PY");
    /// assert_eq!(enigma.ring_settings(), "DEF");
    /// ```
    pub fn ring_settings(&self) -> String {
        self.rotors()
            .map(|r| ((r.ring_setting as u8) + b'A') as char)
            .collect()
    }

    /// Returns a string representing the `Enigma`'s plugboard.
    ///
    /// # Examples
    ///
    /// ```
    /// use ultra::Enigma;
    ///
    /// let enigma = Enigma::new("123", "ABC", "DEF", 'B', "PY");
    /// assert_eq!(enigma.plugboard(), "PY");
    /// ```
    pub fn plugboard(&self) -> String {
        self.plugboard.to_string()
    }

    /// Returns an iterator over the slow, middle, and fast rotors.
    fn rotors(&self) -> impl Iterator<Item=Rotor> {
        vec![self.slow.clone(), self.mid.clone(), self.fast.clone()].into_iter()
    }
}

impl fmt::Display for Enigma {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Rotors: {} / Key: {} / Ring: {} / Plugs: {}",
            self.rotor_list().chars().join("-"),
            self.key_settings().chars().join("-"),
            self.ring_settings().chars().join("-"),
            self.plugboard)
    }
}


#[cfg(test)]
mod tests {
    use super::Enigma;

    #[test]
    fn symmetrical_behaviour() {
        let msg = "THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG".repeat(10);

        for _ in 0..10 {
            let mut enigma = Enigma::random();
            let ciphertext = enigma.encrypt(&msg);

            enigma.reset();
            let plaintext = enigma.encrypt(&ciphertext);

            assert_eq!(plaintext, msg);
        }
    }

    #[test]
    fn identical_from_same_seed() {
        let mut enigma_1 = Enigma::random_from_u64_seed(42);
        let mut enigma_2 = Enigma::random_from_u64_seed(42);
        assert_eq!(enigma_1.encrypt("ENIGMA"), enigma_2.encrypt("ENIGMA"));
    }

    #[test]
    fn case_insensitive_encryption() {
        let mut enigma = Enigma::new("123", "AAA", "AAA", 'B', "");
        let ciphertext1 = enigma.encrypt("Test Message");

        enigma.reset();
        let ciphertext2 = enigma.encrypt("TEST MESSAGE");

        assert_eq!(ciphertext1, ciphertext2);
    }

    #[test]
    fn key_settings() {
        let mut enigma = Enigma::new("123", "CAT", "AAA", 'B', "");
        assert_eq!(enigma.encrypt("AAAAA"), "XLEPK");
    }

    #[test]
    fn ring_settings() {
        let mut enigma = Enigma::new("123", "AAA", "DOG", 'B', "");
        assert_eq!(enigma.encrypt("AAAAA"), "XKJZE");
    }

    #[test]
    fn repetition_period() {
        // Due to the double-rotation of the middle rotor, the Enigma
        // has a period of 26 * 25 * 26 rather the expected 26^3.
        let mut enigma = Enigma::new("123", "AAA", "AAA", 'B', "");

        for _ in 0..(26 * 25 * 26) {
            enigma.advance();
        }

        assert_eq!(enigma.slow.offset, 0);
        assert_eq!(enigma.mid.offset, 0);
        assert_eq!(enigma.fast.offset, 0);
    }

    #[test]
    #[should_panic]
    fn invalid_rotor_count() {
        Enigma::new("12", "AAA", "AAA", 'B', "");
    }
}

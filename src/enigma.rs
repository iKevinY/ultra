use std::ascii::AsciiExt;

use plugboard::Plugboard;
use reflector::Reflector;
use rotor::Rotor;

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
    /// use ultra::enigma::Enigma;
    ///
    /// let mut enigma = Enigma::new("123", "ABC", "DEF", 'B', "PY");
    /// println!("{}", enigma.encrypt("ENIGMA"));
    /// ```
    pub fn new(rotors: &str, keys: &str, rings: &str, reflector: char, plugboard: &str) -> Enigma {
        let rotors: Vec<usize> = rotors
            .chars()
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


    /// Encrypts an entire message, advancing the rotors of the machine
    /// after each alphabetic character is encrypted.
    pub fn encrypt(&mut self, msg: &str) -> String {
        msg.chars().map(|c| self.encrypt_char(c)).collect()
    }

    /// Advances the rotors then returns the substitution of
    /// a character, if the input character was alphabetic.
    pub fn encrypt_char(&mut self, c: char) -> char {
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
        if !c.is_alphabetic() {
            return c;
        }

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
        if self.mid.notch_position() {
            self.mid.advance();
            self.slow.advance();
        } else if self.fast.notch_position() {
            self.mid.advance();
        }

        // Finally, advance the fast rotor
        self.fast.advance();
    }

    /// Resets the `Enigma` to its initial state.
    pub fn reset(&mut self) {
        self.slow.reset();
        self.mid.reset();
        self.fast.reset();
    }
}


#[cfg(test)]
mod tests {
    use super::Enigma;

    #[test]
    fn expected_ciphertext() {
        let mut enigma = Enigma::new("123", "BAT", "HTU", 'B', "");
        assert_eq!(enigma.encrypt("AAAAAAAA"), "ZWGJGTOW");
    }

    #[test]
    fn symmetrical_behaviour() {
        let msg = "THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG";

        let mut enigma = Enigma::new("123", "AAA", "AAA", 'B', "AB YZ");
        let ciphertext = enigma.encrypt(msg);

        enigma.reset();
        let plaintext = enigma.encrypt(&ciphertext);

        assert_eq!(plaintext, msg);
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

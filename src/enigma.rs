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
    /// Creates a new `Enigma`, where `slow`, `mid`, and `fast` are integers
    /// ranging from 1-8 (corresponding to rotors I through VIII of the real
    /// Enigma machine), `reflector` is one of `'A'`, `'B'`, or `'C'`, and
    /// `plugboard` is a string of whitespace-delimited pairs of characters.
    pub fn new(slow: usize, mid: usize, fast: usize, reflector: char, plugboard: &str) -> Enigma {
        let reflector = (reflector as usize) - 65;

        Enigma {
            slow: ROTORS[slow - 1].clone(),
            mid: ROTORS[mid - 1].clone(),
            fast: ROTORS[fast - 1].clone(),
            reflector: REFLECTORS[reflector].clone(),
            plugboard: Plugboard::new(plugboard),
        }
    }


    /// Encrypts an entire message, advancing the rotors of the machine
    /// after each alphabetic character is encrypted.
    pub fn encrypt(&mut self, msg: &str) -> String {
        msg.chars().map(|c| self.encrypt_char(c)).collect()
    }

    /// Returns the substitution of a character, and advances
    /// the rotors if the input character was alphabetic.
    pub fn encrypt_char(&mut self, c: char) -> char {
        if !c.is_ascii() || !c.is_alphabetic() {
            return c;
        }

        let encrypted_char = self.substitute(c.to_ascii_uppercase());
        self.advance();
        encrypted_char
    }


    /// Returns the substitution of a character, which is determined by
    /// passing the character in sequence through the plugboard, the rotors
    /// from `fast` to `slow`, through the reflector, inverted through the
    /// rotors from `slow` to `fast`, and finally through the plugboard.
    fn substitute(&self, c: char) -> char {
        if !c.is_alphabetic() {
            return c;
        }

        let c = self.plugboard.map(c);
        let c = self.slow.substitute(self.mid.substitute(self.fast.substitute(c)));
        let c = self.reflector.reflect(c);
        let c = self.fast.invert(self.mid.invert(self.slow.invert(c)));
        self.plugboard.map(c)
    }

    /// Advances the `fast` rotor, and also advances the
    /// `mid` and `slow` rotors if appropriate.
    fn advance(&mut self) {
        if self.fast.advance() {
            if self.mid.advance() {
                self.slow.advance();
            }
        }
    }
}


lazy_static! {
    static ref ROTORS: Vec<Rotor> = vec![
        Rotor::new("EKMFLGDQVZNTOWYHXUSPAIBRCJ", "Q"),
        Rotor::new("AJDKSIRUXBLHWTMCQGZNPYFVOE", "E"),
        Rotor::new("BDFHJLCPRTXVZNYEIWGAKMUSQO", "V"),
        Rotor::new("ESOVPZJAYQUIRHXLNFTGKDCMWB", "J"),
        Rotor::new("VZBRGITYUPSDNHLXAWMJQOFECK", "Z"),
        Rotor::new("JPGVOUMFYQBENHZRDKASXLICTW", "ZM"),
        Rotor::new("NZJHGRCXMYSWBOUFAIVLPEKQDT", "ZM"),
        Rotor::new("FKQHTLXOCBJSPDZRAMEWNIUYGV", "ZM"),
    ];

    static ref REFLECTORS: Vec<Reflector> = vec![
        Reflector::new("EJMZALYXVBWFCRQUONTSPIKHGD"),
        Reflector::new("YRUHQSLDPXNGOKMIEBFZCWVJAT"),
        Reflector::new("FVPJIAOYEDRZXWGCTKUQSBNMHL"),
    ];
}


#[cfg(test)]
mod tests {
    use super::Enigma;

    #[test]
    fn symmetrical_behaviour() {
        let msg = "THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG";

        let mut enigma = Enigma::new(1, 2, 3, 'B', "AB YZ");
        let ciphertext = enigma.encrypt(msg);

        // Reset the machine to its original state
        let mut enigma = Enigma::new(1, 2, 3, 'B', "AB YZ");
        let plaintext = enigma.encrypt(&ciphertext);

        assert_eq!(plaintext, msg);
    }

    #[test]
    fn case_insensitive_encryption() {
        let mut enigma = Enigma::new(1, 2, 3, 'B', "");
        let ciphertext1 = enigma.encrypt("Test Message");

        let mut enigma = Enigma::new(1, 2, 3, 'B', "");
        let ciphertext2 = enigma.encrypt("TEST MESSAGE");

        assert_eq!(ciphertext1, ciphertext2);
    }
}

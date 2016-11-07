use reflector::Reflector;
use rotor::Rotor;

#[derive(Clone, Debug)]
pub struct Enigma {
    slow: Rotor,
    mid: Rotor,
    fast: Rotor,
    reflector: Reflector,
}

impl Enigma {
    /// Creates a new `Enigma`, where `slow`, `mid`, and `fast` are integers
    /// ranging from 1-8 (corresponding to rotors I through VIII of the real
    /// Enigma machine), and `reflector` is one of `'A'`, `'B'`, or `'C'`.
    pub fn new(slow: usize, mid: usize, fast: usize, reflector: char) -> Enigma {
        let reflector = (reflector as usize) - 65;

        Enigma {
            slow: ROTORS[slow - 1].clone(),
            mid: ROTORS[mid - 1].clone(),
            fast: ROTORS[fast - 1].clone(),
            reflector: REFLECTORS[reflector].clone(),
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
        let encrypted_char = self.substitute(c);

        if c.is_alphabetic() {
            self.advance();
        }

        encrypted_char
    }


    /// Returns the substitution of a character, which is determined
    /// by passing the character in sequence through rotors from
    /// `fast` to `slow`, through the reflector, then inverted
    /// through the rotors from `slow` to `fast`.
    fn substitute(&self, c: char) -> char {
        if !c.is_alphabetic() {
            return c;
        }

        let c = self.slow.substitute(self.mid.substitute(self.fast.substitute(c)));
        let c = self.reflector.reflect(c);
        self.fast.invert(self.mid.invert(self.slow.invert(c)))
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

        let mut enigma = Enigma::new(1, 2, 3, 'B');
        let ciphertext = enigma.encrypt(msg);

        // Reset the machine to its original state
        let mut enigma = Enigma::new(1, 2, 3, 'B');
        let plaintext = enigma.encrypt(&ciphertext);

        assert_eq!(plaintext, msg);
    }
}

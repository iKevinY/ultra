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
    /// Creates a new `Enigma`.
    pub fn new(slow: usize, mid: usize, fast: usize, reflector: char) -> Enigma {
        let reflector = (reflector as usize) - 65;

        Enigma {
            slow: ROTORS[slow - 1].clone(),
            mid: ROTORS[mid - 1].clone(),
            fast: ROTORS[fast - 1].clone(),
            reflector: REFLECTORS[reflector].clone(),
        }
    }

    /// Advances the `fast` rotor of the `Enigma`, and also advances
    /// the `mid` and `slow` rotors if appropriate.
    pub fn advance(&mut self) {
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
    fn advance_enigma() {
        let mut enigma = Enigma::new(1, 2, 3, 'B');
        enigma.advance();
    }
}

use super::{CharIndex, ToChar};
use constants::{ROTORS, NOTCHES};

#[derive(Clone, Debug)]
pub struct Rotor {
    mapping: Vec<char>,
    inverse: Vec<char>,
    notches: Vec<usize>,
    pub offset: usize,
    key_setting: usize,
    ring_setting: usize,
}

impl Rotor {
    /// Creates a new `Rotor`, where `mapping` is a 26-character `&str`
    /// containing some ordering of all letters in the alphabet, `notches`
    /// is a `&str` where each character in the string corresponds to a
    /// single notch in the rotor, and `key` and `ring` are the rotor's
    /// key and ring settings respectively.
    pub fn new(mapping: &str, notches: &str, key: char, ring: char) -> Rotor {
        let mapping: Vec<char> = mapping.chars().collect();

        if mapping.len() != 26 {
            panic!("Rotor mappings must be 26 characters long.");
        }

        let mut inverse = vec!['A'; 26];

        for (i, &c) in mapping.iter().enumerate() {
            inverse[c.index() % 26] = i.to_char();
        }

        Rotor {
            mapping: mapping,
            inverse: inverse,
            notches: notches.chars().map(|c| c.index()).collect(),
            offset: key.index(),
            key_setting: key.index(),
            ring_setting: ring.index(),
        }
    }

    /// Creates a new `Rotor`, where `num` is a number from 1-8 corresponding
    /// to rotors I through VIII of the Enigma machine, and `key` and `ring`
    /// are the rotor's key and ring settings respectively.
    pub fn from_enigma(num: usize, key: char, ring: char) -> Rotor {
        Rotor::new(ROTORS[num - 1], NOTCHES[num - 1], key, ring)
    }

    fn map(&self, c: char, mapping: &[char]) -> char {
        let offset = 26 + self.offset - self.ring_setting;
        let index = mapping[(c.index() + offset) % 26].index();
        (52 + index - offset).to_char()
    }

    /// Returns the substitution of a given character
    /// based on the current offset of the rotor.
    pub fn substitute(&self, c: char) -> char {
        self.map(c, &self.mapping)
    }

    /// Returns the substitution of a given character when run through
    /// the rotor in reverse (on the path back from the reflector).
    pub fn invert(&self, c: char) -> char {
        self.map(c, &self.inverse)
    }

    /// Advances this rotor one position.
    pub fn advance(&mut self) {
        self.offset = (self.offset + 1) % 26;
    }

    /// Returns true if the rotor is currently in a notch position.
    pub fn at_notch(&self) -> bool {
        self.notches.iter().any(|&n| n == self.offset)
    }

    /// Resets the rotor to its initial state.
    pub fn reset(&mut self) {
        self.offset = self.key_setting;
    }
}


#[cfg(test)]
mod tests {
    use super::Rotor;

    #[test]
    fn char_substitution() {
        let rotor = Rotor::from_enigma(1, 'A', 'A');
        assert_eq!(rotor.substitute('A'), 'E');
        assert_eq!(rotor.substitute('B'), 'K');
    }

    #[test]
    fn step_rotor() {
        let mut rotor = Rotor::from_enigma(1, 'A', 'A');
        assert_eq!(rotor.substitute('A'), 'E');

        rotor.advance();
        assert_eq!(rotor.offset, 1);
        assert_eq!(rotor.substitute('A'), 'J');

        rotor.advance();
        assert_eq!(rotor.offset, 2);
        assert_eq!(rotor.substitute('A'), 'K');
    }

    #[test]
    fn step_inverse() {
        let mut rotor = Rotor::from_enigma(1, 'A', 'A');
        assert_eq!(rotor.invert('E'), 'A');
        rotor.advance();
        assert_eq!(rotor.invert('K'), 'D');
        rotor.advance();
        assert_eq!(rotor.invert('M'), 'K');
    }

    #[test]
    fn key_setting() {
        let rotor = Rotor::from_enigma(1, 'D', 'A');
        assert_eq!(rotor.offset, 3)
    }

    #[test]
    fn ring_setting() {
        let rotor = Rotor::from_enigma(1, 'A', 'B');
        assert_eq!(rotor.substitute('A'), 'K');
    }

    #[test]
    fn reset_rotor() {
        let mut rotor = Rotor::from_enigma(1, 'A', 'A');
        assert_eq!(rotor.offset, 0);

        for _ in 0..10 {
            rotor.advance();
        }

        assert_eq!(rotor.offset, 10);
        rotor.reset();
        assert_eq!(rotor.offset, 0);
    }

    #[test]
    fn inverse_mapping() {
        let rotor = Rotor::from_enigma(1, 'A', 'A');
        let inverse: String = rotor.inverse.into_iter().collect();
        assert_eq!(&inverse, "UWYGADFPVZBECKMTHXSLRINQOJ");
    }

    #[test]
    fn matching_inverses() {
        let mut rotor = Rotor::from_enigma(1, 'A', 'A');
        for i in b'A'..(b'Z' + 1) {
            let c = i as char;
            assert_eq!(c, rotor.invert(rotor.substitute(c)));
            rotor.advance();
        }
    }

    #[test]
    #[should_panic]
    fn invalid_rotor() {
        Rotor::new("ABC", "A", 'A', 'A');
    }
}
